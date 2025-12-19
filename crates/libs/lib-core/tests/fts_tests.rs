use lib_core::Result;
use lib_core::ctx::Ctx;
use lib_core::model::ModelManager;
use lib_core::model::agent::{AgentBmc, AgentForCreate};
use lib_core::model::message::{MessageBmc, MessageForCreate};
use lib_core::model::project::ProjectBmc;

// --- Test Setup Helper ---
async fn create_test_mm() -> ModelManager {
    // In-memory DB with migrations
    ModelManager::new().await.unwrap()
}

async fn setup_project_and_agent(ctx: &Ctx, mm: &ModelManager, suffix: &str) -> (i64, i64) {
    let p_slug = format!("fts-proj-{}", suffix);
    let p_id = ProjectBmc::create(ctx, mm, &p_slug, "FTS Project")
        .await
        .unwrap();

    let a_id = AgentBmc::create(
        ctx,
        mm,
        AgentForCreate {
            project_id: p_id,
            name: format!("agent-{}", suffix),
            program: "test".into(),
            model: "test".into(),
            task_description: "test".into(),
        },
    )
    .await
    .unwrap();

    (p_id, a_id)
}

#[tokio::test]
async fn test_fts_wildcard_search() -> Result<()> {
    let mm = create_test_mm().await;
    let ctx = Ctx::root_ctx();
    let (p_id, a_id) = setup_project_and_agent(&ctx, &mm, "wild").await;

    // 1. Create a message with specific word
    MessageBmc::create(
        &ctx,
        &mm,
        MessageForCreate {
            project_id: p_id,
            sender_id: a_id,
            recipient_ids: vec![],
            cc_ids: None,
            bcc_ids: None,
            subject: "The quick brown fox".to_string(),
            body_md: "Jumps over the lazy dog".to_string(),
            thread_id: None,
            importance: None,
            ack_required: false,
        },
    )
    .await?;

    // 2. Search with prefix wildcard (standard FTS5)
    // "quick*" should match "quick" if we allow wildcards
    // Currently fails because we quote it as "quick*"
    let res = MessageBmc::search(&ctx, &mm, p_id, "quick*", 10).await?;
    assert_eq!(res.len(), 1, "Should match 'quick*' (prefix)");

    // 3. Search with phrase to ensure we don't break normal phrases
    let res2 = MessageBmc::search(&ctx, &mm, p_id, "\"brown fox\"", 10).await?;
    assert_eq!(res2.len(), 1, "Should match phrase \"brown fox\"");

    Ok(())
}

#[tokio::test]
async fn test_fts_malformed_query_graceful() -> Result<()> {
    let mm = create_test_mm().await;
    let ctx = Ctx::root_ctx();
    let (p_id, _) = setup_project_and_agent(&ctx, &mm, "err").await;

    // Unclosed quote - FTS5 throws error if passed raw
    // We want graceful empty result (or handled error, but preferably empty as per python logic)
    // Currently passes because we quote it, making it valid literal.
    // When we fix wildcards (unquote), this will likely bubble an error if not handled.
    let res = MessageBmc::search(&ctx, &mm, p_id, "\"unclosed phrase", 10).await;

    // We expect OK (handled) and empty
    assert!(res.is_ok(), "Should return Ok for malformed FTS query");
    let messages = res.unwrap();
    assert!(
        messages.is_empty(),
        "Should return empty list for malformed query"
    );

    Ok(())
}
