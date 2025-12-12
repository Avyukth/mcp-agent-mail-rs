use e2e_tests::TestConfig;
use jugar_probar::Browser;
use std::time::Duration;

#[tokio::test]
async fn test_video_walkthrough_navigation() {
    // MASTER VIDEO WALKTHROUGH DRIVER
    // This script helps automate the navigation for the video walkthrough.
    // It assumes a local backend is running at port 8765 and frontend at 4090.
    // Run this test while recording the screen, ideally enabling headful mode via env vars if supported.
    //
    // FLOW:
    // 1. Dashboard (Light/Dark Toggle)
    // 2. Projects (Create/List)
    // 3. Agents (Search/List)
    // 4. Inbox (Compose, High Priority, Reply, Cancel)

    let mut config = TestConfig::default();
    config.web_ui_url = "http://localhost:4090".to_string();

    println!("🎥 STARTING VIDEO WALKTHROUGH SCRIPT...");
    let browser = Browser::launch(Default::default()).await.expect("Failed to launch browser");
    let mut page = browser.new_page().await.expect("Failed to create page");

    // SCENE 1: DASHBOARD
    println!("🎬 SCENE 1: Dashboard & Theme");
    page.goto(&config.web_ui_url).await.expect("Failed to load Dashboard");
    println!("   ACTION: Show Dashboard. Toggle Theme (Moon Icon). Check Recent Activity.");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // SCENE 2: PROJECTS
    println!("🎬 SCENE 2: Projects");
    let projects_url = format!("{}/projects", config.web_ui_url);
    page.goto(&projects_url).await.expect("Failed to load Projects");
    println!("   ACTION: Create Project '/tmp/demo'. Verify List.");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // SCENE 3: AGENTS
    println!("🎬 SCENE 3: Agents");
    let agents_url = format!("{}/agents", config.web_ui_url);
    page.goto(&agents_url).await.expect("Failed to load Agents");
    println!("   ACTION: Search for 'Alice'. Scroll list.");
    tokio::time::sleep(Duration::from_secs(5)).await;

    // SCENE 4: INBOX & COMPOSE
    println!("🎬 SCENE 4: Inbox - Core Feature");
    let inbox_url = format!("{}/inbox", config.web_ui_url);
    page.goto(&inbox_url).await.expect("Failed to load Inbox");
    println!("   ACTION: Select 'Demo Project' & 'Alice'.");
    println!("   ACTION: Click Compose -> To: Bob, Subject: 'System Alert', Priority: High. SEND.");
    tokio::time::sleep(Duration::from_secs(8)).await;

    // SCENE 5: REPLY & CANCEL
    println!("🎬 SCENE 5: Reply & Cancel");
    println!("   ACTION: Click the new message. Click Reply. Send.");
    println!("   ACTION: Click Compose. Type nonsense. Click Cancel.");
    tokio::time::sleep(Duration::from_secs(8)).await;

    println!("✅ CUT! Walkthrough complete.");
}
