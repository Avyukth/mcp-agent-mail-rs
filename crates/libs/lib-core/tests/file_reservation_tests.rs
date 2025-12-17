//! File reservation model tests
//!
//! Tests for file reservation CRUD operations - critical for multi-agent coordination.

#[path = "common/mod.rs"]
mod common;

use crate::common::TestContext;
use chrono::{Duration, Utc};
use lib_core::model::agent::{AgentBmc, AgentForCreate};
use lib_core::model::file_reservation::{FileReservationBmc, FileReservationForCreate};
use lib_core::model::project::ProjectBmc;
use lib_core::utils::slugify;

/// Helper to set up a project and agent for file reservation tests
async fn setup_project_and_agent(tc: &TestContext) -> (i64, i64) {
    let human_key = "/test/repo";
    let slug = slugify(human_key);

    let project_id = ProjectBmc::create(&tc.ctx, &tc.mm, &slug, human_key)
        .await
        .expect("Failed to create project");

    let agent = AgentForCreate {
        project_id,
        name: "test-agent".to_string(),
        program: "claude-code".to_string(),
        model: "claude-3".to_string(),
        task_description: "Testing file reservations".to_string(),
    };

    let agent_id = AgentBmc::create(&tc.ctx, &tc.mm, agent)
        .await
        .expect("Failed to create agent");

    (project_id, agent_id)
}

/// Test creating a file reservation
#[tokio::test]
async fn test_create_file_reservation() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "src/**/*.rs".to_string(),
        exclusive: true,
        reason: "Editing source files".to_string(),
        expires_ts,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create file reservation");

    assert!(reservation_id > 0, "Reservation ID should be positive");
}

/// Test getting a file reservation by ID
#[tokio::test]
async fn test_get_file_reservation() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(2);
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "Cargo.toml".to_string(),
        exclusive: true,
        reason: "Updating dependencies".to_string(),
        expires_ts,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create file reservation");

    let reservation = FileReservationBmc::get(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to get file reservation");

    assert_eq!(reservation.id, reservation_id);
    assert_eq!(reservation.project_id, project_id);
    assert_eq!(reservation.agent_id, agent_id);
    assert_eq!(reservation.path_pattern, "Cargo.toml");
    assert!(reservation.exclusive);
    assert_eq!(reservation.reason, "Updating dependencies");
    assert!(reservation.released_ts.is_none());
}

/// Test listing active file reservations for a project
#[tokio::test]
async fn test_list_active_file_reservations() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    // Create multiple reservations
    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);
    for pattern in &["src/*.rs", "tests/*.rs", "docs/*.md"] {
        let fr_c = FileReservationForCreate {
            project_id,
            agent_id,
            path_pattern: pattern.to_string(),
            exclusive: true,
            reason: "Testing".to_string(),
            expires_ts,
        };
        FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
            .await
            .expect("Failed to create reservation");
    }

    let active = FileReservationBmc::list_active_for_project(&tc.ctx, &tc.mm, project_id)
        .await
        .expect("Failed to list active reservations");

    assert_eq!(active.len(), 3, "Should have 3 active reservations");
}

/// Test releasing a file reservation
#[tokio::test]
async fn test_release_file_reservation() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "README.md".to_string(),
        exclusive: false,
        reason: "Updating docs".to_string(),
        expires_ts,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create reservation");

    // Release the reservation
    FileReservationBmc::release(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to release reservation");

    // Verify it's released
    let reservation = FileReservationBmc::get(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to get reservation");

    assert!(
        reservation.released_ts.is_some(),
        "Reservation should have released_ts set"
    );

    // Active list should be empty
    let active = FileReservationBmc::list_active_for_project(&tc.ctx, &tc.mm, project_id)
        .await
        .expect("Failed to list active reservations");

    assert_eq!(
        active.len(),
        0,
        "Should have no active reservations after release"
    );
}

/// Test releasing a file reservation by path
#[tokio::test]
async fn test_release_by_path() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);
    let path_pattern = "lib/**/*.rs";
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: path_pattern.to_string(),
        exclusive: true,
        reason: "Refactoring".to_string(),
        expires_ts,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create reservation");

    // Release by path
    let released_id =
        FileReservationBmc::release_by_path(&tc.ctx, &tc.mm, project_id, agent_id, path_pattern)
            .await
            .expect("Failed to release by path");

    assert_eq!(
        released_id,
        Some(reservation_id),
        "Should return the released reservation ID"
    );
}

/// Test force releasing a file reservation
#[tokio::test]
async fn test_force_release() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "config/*.yaml".to_string(),
        exclusive: true,
        reason: "Config update".to_string(),
        expires_ts,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create reservation");

    // Force release (emergency override)
    FileReservationBmc::force_release(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to force release");

    // Verify it's released
    let reservation = FileReservationBmc::get(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to get reservation");

    assert!(
        reservation.released_ts.is_some(),
        "Should be released after force_release"
    );
}

/// Test renewing a file reservation
#[tokio::test]
async fn test_renew_file_reservation() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let original_expires = Utc::now().naive_utc() + Duration::hours(1);
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "build/**".to_string(),
        exclusive: true,
        reason: "Build process".to_string(),
        expires_ts: original_expires,
    };

    let reservation_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create reservation");

    // Renew with extended time
    let new_expires = Utc::now().naive_utc() + Duration::hours(3);
    FileReservationBmc::renew(&tc.ctx, &tc.mm, reservation_id, new_expires)
        .await
        .expect("Failed to renew reservation");

    // Verify the new expiry time (note: datetime comparison may have precision issues)
    let reservation = FileReservationBmc::get(&tc.ctx, &tc.mm, reservation_id)
        .await
        .expect("Failed to get reservation");

    // Check that expires_ts was updated (should be later than original)
    assert!(
        reservation.expires_ts > original_expires,
        "Expiry should be extended after renewal"
    );
}

/// Test listing all reservations (including released)
#[tokio::test]
async fn test_list_all_for_project() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (project_id, agent_id) = setup_project_and_agent(&tc).await;

    let expires_ts = Utc::now().naive_utc() + Duration::hours(1);

    // Create and release one reservation
    let fr_c = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "old/*.rs".to_string(),
        exclusive: false,
        reason: "Old work".to_string(),
        expires_ts,
    };
    let released_id = FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c)
        .await
        .expect("Failed to create reservation");
    FileReservationBmc::release(&tc.ctx, &tc.mm, released_id)
        .await
        .expect("Failed to release");

    // Create an active reservation
    let fr_c2 = FileReservationForCreate {
        project_id,
        agent_id,
        path_pattern: "new/*.rs".to_string(),
        exclusive: true,
        reason: "New work".to_string(),
        expires_ts,
    };
    FileReservationBmc::create(&tc.ctx, &tc.mm, fr_c2)
        .await
        .expect("Failed to create reservation");

    // list_all_for_project should return both
    let all = FileReservationBmc::list_all_for_project(&tc.ctx, &tc.mm, project_id)
        .await
        .expect("Failed to list all reservations");

    assert_eq!(
        all.len(),
        2,
        "Should have 2 total reservations (1 released, 1 active)"
    );

    // list_active should return only 1
    let active = FileReservationBmc::list_active_for_project(&tc.ctx, &tc.mm, project_id)
        .await
        .expect("Failed to list active reservations");

    assert_eq!(active.len(), 1, "Should have 1 active reservation");
}

/// Test file reservation not found error
#[tokio::test]
async fn test_file_reservation_not_found() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let result = FileReservationBmc::get(&tc.ctx, &tc.mm, 99999).await;

    assert!(
        result.is_err(),
        "Should return error for nonexistent reservation"
    );
}
