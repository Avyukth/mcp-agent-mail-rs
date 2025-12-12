#![allow(unused)]
use crate::common::TestContext;
use lib_core::model::macro_def::{MacroDefBmc, MacroDefForCreate};
use lib_core::model::project::ProjectBmc;

mod common;

#[tokio::test]
async fn test_builtin_macros_registration() {
    let ctx = TestContext::new().await.unwrap();
    let mm = &ctx.mm;
    let c = &ctx.ctx;

    // Create a project
    let project_id = ProjectBmc::create(c, mm, "macro-test-project", "/path/to/project").await.unwrap();

    // Verify automatic registration via ProjectBmc::create
    let listed = MacroDefBmc::list(c, mm, project_id).await.unwrap();
    assert_eq!(listed.len(), 5, "ProjectBmc::create should have registered 5 built-in macros");
    
    // built-in macros names to check
    let names: Vec<String> = listed.into_iter().map(|m| m.name).collect();
    assert!(names.contains(&"start_session".to_string()));
    assert!(names.contains(&"prepare_thread".to_string()));
    assert!(names.contains(&"file_reservation_cycle".to_string()));
    assert!(names.contains(&"contact_handshake".to_string()));
    assert!(names.contains(&"broadcast_message".to_string()));

    // Verify idempotency
    let created_again = MacroDefBmc::ensure_builtin_macros(c, mm, project_id).await.unwrap();
    assert_eq!(created_again.len(), 0, "Calling ensure_builtin_macros again should create 0 new macros");
}

#[tokio::test]
async fn test_macro_crud() {
    let ctx = TestContext::new().await.unwrap();
    let mm = &ctx.mm;
    let c = &ctx.ctx;

    let project_id = ProjectBmc::create(c, mm, "crud-test", "/tmp/crud").await.unwrap();

    // Create
    let macro_c = MacroDefForCreate {
        project_id,
        name: "custom_macro".to_string(),
        description: "A custom test macro".to_string(),
        steps: vec![serde_json::json!({"action": "test"})],
    };
    let mid = MacroDefBmc::create(c, mm, macro_c).await.unwrap();

    // Get
    let m = MacroDefBmc::get_by_name(c, mm, project_id, "custom_macro").await.unwrap();
    assert_eq!(m.id, mid);
    assert_eq!(m.description, "A custom test macro");

    // List
    let list = MacroDefBmc::list(c, mm, project_id).await.unwrap();
    assert!(list.iter().any(|x| x.name == "custom_macro"));

    // Delete
    let deleted = MacroDefBmc::delete(c, mm, project_id, "custom_macro").await.unwrap();
    assert!(deleted);

    // Verify gone
    let list_after = MacroDefBmc::list(c, mm, project_id).await.unwrap();
    assert!(!list_after.iter().any(|x| x.name == "custom_macro"));
}
