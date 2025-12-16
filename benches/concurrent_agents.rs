//! Concurrent Agents Benchmark POC
//!
//! Tests system performance with > 10 concurrent agents sending messages.
//!
//! Usage:
//!   cargo run --release --bin concurrent-agents-bench -- [OPTIONS]
//!
//! Options:
//!   --agents N      Number of concurrent agents (default: 20)
//!   --messages N    Messages per agent (default: 10)
//!   --url URL       Server URL (default: http://localhost:8765)

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

const DEFAULT_AGENTS: usize = 20;
const DEFAULT_MESSAGES_PER_AGENT: usize = 10;
const DEFAULT_URL: &str = "http://localhost:8765";

#[derive(Debug, Serialize)]
struct EnsureProjectRequest {
    human_key: String,
}

#[derive(Debug, Serialize)]
struct RegisterAgentRequest {
    project_slug: String,
    name: String,
    program: String,
    model: String,
}

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    project_slug: String,
    sender_name: String,
    recipient_names: Vec<String>,
    subject: String,
    body_md: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    importance: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EnsureProjectResponse {
    slug: String,
}


#[derive(Debug)]
struct BenchmarkStats {
    total_messages: u64,
    successful: u64,
    failed: u64,
    total_duration_ms: u64,
    min_latency_ms: u64,
    max_latency_ms: u64,
    avg_latency_ms: f64,
    p50_latency_ms: u64,
    p95_latency_ms: u64,
    p99_latency_ms: u64,
}

#[derive(Debug)]
struct AtomicStats {
    successful: AtomicU64,
    failed: AtomicU64,
    latencies: tokio::sync::Mutex<Vec<u64>>,
}

impl AtomicStats {
    fn new() -> Self {
        Self {
            successful: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            latencies: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    async fn record_success(&self, latency_ms: u64) {
        self.successful.fetch_add(1, Ordering::Relaxed);
        self.latencies.lock().await.push(latency_ms);
    }

    fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    async fn finalize(self, total_duration: Duration) -> BenchmarkStats {
        let successful = self.successful.load(Ordering::Relaxed);
        let failed = self.failed.load(Ordering::Relaxed);
        let mut latencies = self.latencies.into_inner();
        latencies.sort_unstable();

        let total_messages = successful + failed;
        let (min, max, avg, p50, p95, p99) = if !latencies.is_empty() {
            let min = *latencies.first().unwrap();
            let max = *latencies.last().unwrap();
            let avg = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
            let p50 = latencies[latencies.len() * 50 / 100];
            let p95 = latencies[latencies.len() * 95 / 100];
            let p99 = latencies[latencies.len().saturating_sub(1).max(latencies.len() * 99 / 100)];
            (min, max, avg, p50, p95, p99)
        } else {
            (0, 0, 0.0, 0, 0, 0)
        };

        BenchmarkStats {
            total_messages,
            successful,
            failed,
            total_duration_ms: total_duration.as_millis() as u64,
            min_latency_ms: min,
            max_latency_ms: max,
            avg_latency_ms: avg,
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse args
    let args: Vec<String> = std::env::args().collect();
    let mut num_agents = DEFAULT_AGENTS;
    let mut messages_per_agent = DEFAULT_MESSAGES_PER_AGENT;
    let mut base_url = DEFAULT_URL.to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--agents" => {
                num_agents = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_AGENTS);
                i += 2;
            }
            "--messages" => {
                messages_per_agent = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(DEFAULT_MESSAGES_PER_AGENT);
                i += 2;
            }
            "--url" => {
                base_url = args.get(i + 1).cloned().unwrap_or_else(|| DEFAULT_URL.to_string());
                i += 2;
            }
            "--help" | "-h" => {
                println!("Concurrent Agents Benchmark POC");
                println!();
                println!("Usage: concurrent-agents-bench [OPTIONS]");
                println!();
                println!("Options:");
                println!("  --agents N      Number of concurrent agents (default: {})", DEFAULT_AGENTS);
                println!("  --messages N    Messages per agent (default: {})", DEFAULT_MESSAGES_PER_AGENT);
                println!("  --url URL       Server URL (default: {})", DEFAULT_URL);
                return Ok(());
            }
            _ => i += 1,
        }
    }

    println!("=== Concurrent Agents Benchmark POC ===");
    println!();
    println!("Configuration:");
    println!("  Server URL:         {}", base_url);
    println!("  Concurrent agents:  {}", num_agents);
    println!("  Messages per agent: {}", messages_per_agent);
    println!("  Total messages:     {}", num_agents * messages_per_agent);
    println!();

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    // Setup: Create project
    println!("[1/4] Creating benchmark project...");
    let human_key = format!("bench-concurrent-{}", chrono::Utc::now().timestamp());
    let res = client
        .post(format!("{}/api/project/ensure", base_url))
        .json(&EnsureProjectRequest {
            human_key: human_key.clone(),
        })
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await?;
        anyhow::bail!("Failed to create project: {}", text);
    }
    let project: EnsureProjectResponse = res.json().await?;
    let project_slug = project.slug;
    println!("  Project '{}' created", project_slug);

    // Setup: Register agents
    println!("[2/4] Registering {} agents...", num_agents);
    let agent_names: Vec<String> = (0..num_agents)
        .map(|i| format!("agent-{:03}", i))
        .collect();

    for name in &agent_names {
        let res = client
            .post(format!("{}/api/agent/register", base_url))
            .json(&RegisterAgentRequest {
                project_slug: project_slug.clone(),
                name: name.clone(),
                program: "benchmark".to_string(),
                model: "bench-model".to_string(),
            })
            .send()
            .await?;

        if !res.status().is_success() {
            let text = res.text().await?;
            eprintln!("  Warning: Failed to register {}: {}", name, text);
        }
    }
    println!("  {} agents registered", num_agents);

    // Run benchmark
    println!("[3/4] Running benchmark...");
    println!("  Sending {} messages with {} concurrent agents", num_agents * messages_per_agent, num_agents);

    let stats = Arc::new(AtomicStats::new());
    let semaphore = Arc::new(Semaphore::new(num_agents)); // Limit concurrent connections

    let start = Instant::now();
    let mut handles = Vec::new();

    for agent_idx in 0..num_agents {
        let client = client.clone();
        let base_url = base_url.clone();
        let project_slug = project_slug.clone();
        let agent_name = agent_names[agent_idx].clone();
        let recipient = agent_names[(agent_idx + 1) % num_agents].clone(); // Send to next agent
        let stats = Arc::clone(&stats);
        let semaphore = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            for msg_idx in 0..messages_per_agent {
                let _permit = semaphore.acquire().await.unwrap();

                let msg_start = Instant::now();
                let res = client
                    .post(format!("{}/api/message/send", base_url))
                    .json(&SendMessageRequest {
                        project_slug: project_slug.clone(),
                        sender_name: agent_name.clone(),
                        recipient_names: vec![recipient.clone()],
                        subject: format!("Benchmark message {}-{}", agent_idx, msg_idx),
                        body_md: format!(
                            "This is a benchmark message from {} to {} (message {} of {})",
                            agent_name, recipient, msg_idx + 1, messages_per_agent
                        ),
                        thread_id: None,
                        importance: Some("normal".to_string()),
                    })
                    .send()
                    .await;

                let latency_ms = msg_start.elapsed().as_millis() as u64;

                match res {
                    Ok(response) if response.status().is_success() => {
                        stats.record_success(latency_ms).await;
                    }
                    Ok(response) => {
                        eprintln!("  [{}] Message {} failed: HTTP {}", agent_name, msg_idx, response.status());
                        stats.record_failure();
                    }
                    Err(e) => {
                        eprintln!("  [{}] Message {} error: {}", agent_name, msg_idx, e);
                        stats.record_failure();
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all agents to complete
    for handle in handles {
        handle.await?;
    }

    let total_duration = start.elapsed();
    let stats = Arc::try_unwrap(stats).unwrap().finalize(total_duration).await;

    // Print results
    println!();
    println!("[4/4] Results:");
    println!("  =================================");
    println!("  Total messages:     {}", stats.total_messages);
    println!("  Successful:         {} ({:.1}%)", stats.successful,
        stats.successful as f64 / stats.total_messages as f64 * 100.0);
    println!("  Failed:             {} ({:.1}%)", stats.failed,
        stats.failed as f64 / stats.total_messages as f64 * 100.0);
    println!();
    println!("  Duration:           {:.2}s", stats.total_duration_ms as f64 / 1000.0);
    println!("  Throughput:         {:.1} msg/s",
        stats.successful as f64 / (stats.total_duration_ms as f64 / 1000.0));
    println!();
    println!("  Latency (ms):");
    println!("    Min:              {}ms", stats.min_latency_ms);
    println!("    Avg:              {:.1}ms", stats.avg_latency_ms);
    println!("    P50:              {}ms", stats.p50_latency_ms);
    println!("    P95:              {}ms", stats.p95_latency_ms);
    println!("    P99:              {}ms", stats.p99_latency_ms);
    println!("    Max:              {}ms", stats.max_latency_ms);
    println!("  =================================");

    // Evaluate against targets
    println!();
    println!("  Target Evaluation:");
    let concurrent_pass = num_agents >= 10;
    let latency_p99_pass = stats.p99_latency_ms < 500;
    let error_rate_pass = (stats.failed as f64 / stats.total_messages as f64) < 0.01;

    println!("    [{}] Concurrent agents >= 10: {} agents",
        if concurrent_pass { "PASS" } else { "FAIL" }, num_agents);
    println!("    [{}] P99 latency < 500ms: {}ms",
        if latency_p99_pass { "PASS" } else { "FAIL" }, stats.p99_latency_ms);
    println!("    [{}] Error rate < 1%: {:.2}%",
        if error_rate_pass { "PASS" } else { "FAIL" },
        stats.failed as f64 / stats.total_messages as f64 * 100.0);

    Ok(())
}
