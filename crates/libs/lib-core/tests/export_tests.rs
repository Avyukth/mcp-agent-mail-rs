//! Export model tests
//!
//! Tests for mailbox export functionality in various formats.

// Tests are allowed to use unwrap()/expect() for clearer failure messages
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::inefficient_to_string
)]

#[path = "common/mod.rs"]
mod common;

use crate::common::TestContext;
use lib_core::model::agent::{AgentBmc, AgentForCreate};
use lib_core::model::export::{ExportBmc, ExportFormat, ScrubMode};
use lib_core::model::message::{MessageBmc, MessageForCreate};
use lib_core::model::project::ProjectBmc;
use lib_core::utils::slugify;

/// Helper to set up a project with messages for export tests
async fn setup_project_with_messages(tc: &TestContext, suffix: &str) -> (i64, String) {
    let human_key = format!("/test/export-repo-{}", suffix);
    let slug = slugify(&human_key);

    let project_id = ProjectBmc::create(&tc.ctx, &tc.mm, &slug, &human_key)
        .await
        .expect("Failed to create project");

    // Create sender agent
    let sender = AgentForCreate {
        project_id,
        name: "sender-agent".to_string(),
        program: "claude-code".to_string(),
        model: "claude-3".to_string(),
        task_description: "Sending messages".to_string(),
    };
    let sender_id = AgentBmc::create(&tc.ctx, &tc.mm, sender)
        .await
        .expect("Failed to create sender");

    // Create recipient agent
    let recipient = AgentForCreate {
        project_id,
        name: "recipient-agent".to_string(),
        program: "cursor".to_string(),
        model: "gpt-4".to_string(),
        task_description: "Receiving messages".to_string(),
    };
    let recipient_id = AgentBmc::create(&tc.ctx, &tc.mm, recipient)
        .await
        .expect("Failed to create recipient");

    // Create some messages
    for i in 1..=3 {
        let msg = MessageForCreate {
            project_id,
            sender_id,
            recipient_ids: vec![recipient_id],
            cc_ids: None,
            bcc_ids: None,
            subject: format!("Test Message {}", i),
            body_md: format!("This is the body of message {}.", i),
            thread_id: None,
            importance: None,
            ack_required: false,
        };
        MessageBmc::create(&tc.ctx, &tc.mm, msg)
            .await
            .expect("Failed to create message");
    }

    (project_id, slug)
}

/// Test exporting mailbox in JSON format
#[tokio::test]
async fn test_export_json() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "json").await;

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Json,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert_eq!(exported.project_slug, slug);
    assert_eq!(exported.format, "json");
    assert_eq!(exported.message_count, 3);
    assert!(exported.content.contains("Test Message"));
    assert!(
        exported.content.starts_with('['),
        "JSON should start with array"
    );
}

/// Test exporting mailbox in HTML format
#[tokio::test]
async fn test_export_html() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "html").await;

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Html,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert_eq!(exported.format, "html");
    assert!(exported.content.contains("<!DOCTYPE html>"));
    assert!(exported.content.contains("<title>"));
    assert!(exported.content.contains("Test Message"));
}

/// Test exporting mailbox in Markdown format
#[tokio::test]
async fn test_export_markdown() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "md").await;

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Markdown,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert_eq!(exported.format, "markdown");
    assert!(exported.content.contains("# Mailbox Export"));
    assert!(exported.content.contains("## Test Message"));
    assert!(exported.content.contains("**From:**"));
}

/// Test exporting mailbox in CSV format
#[tokio::test]
async fn test_export_csv() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "csv").await;

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Csv,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert_eq!(exported.format, "csv");
    assert!(
        exported
            .content
            .contains("id,created_at,sender,subject,body")
    );
    assert!(exported.content.contains("Test Message"));
}

/// Test exporting empty mailbox
#[tokio::test]
async fn test_export_empty_mailbox() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let human_key = "/test/empty-export-repo";
    let slug = slugify(human_key);

    ProjectBmc::create(&tc.ctx, &tc.mm, &slug, human_key)
        .await
        .expect("Failed to create project");

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Json,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert_eq!(exported.message_count, 0);
    assert_eq!(exported.content, "[]"); // Empty JSON array
}

/// Test export format parsing
#[tokio::test]
async fn test_export_format_parsing() {
    use std::str::FromStr;

    assert_eq!(ExportFormat::from_str("html").unwrap(), ExportFormat::Html);
    assert_eq!(ExportFormat::from_str("HTML").unwrap(), ExportFormat::Html);
    assert_eq!(ExportFormat::from_str("json").unwrap(), ExportFormat::Json);
    assert_eq!(
        ExportFormat::from_str("md").unwrap(),
        ExportFormat::Markdown
    );
    assert_eq!(
        ExportFormat::from_str("markdown").unwrap(),
        ExportFormat::Markdown
    );
    assert_eq!(ExportFormat::from_str("csv").unwrap(), ExportFormat::Csv);
    // Unknown defaults to JSON
    assert_eq!(
        ExportFormat::from_str("unknown").unwrap(),
        ExportFormat::Json
    );
}

/// Test export for nonexistent project
#[tokio::test]
async fn test_export_nonexistent_project() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let result = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        "nonexistent-slug",
        ExportFormat::Json,
        ScrubMode::None,
        false,
    )
    .await;

    assert!(result.is_err(), "Should fail for nonexistent project");
}

/// Test exported_at timestamp is set
#[tokio::test]
async fn test_export_timestamp() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "timestamp").await;

    let exported = ExportBmc::export_mailbox(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Json,
        ScrubMode::None,
        false,
    )
    .await
    .expect("Failed to export mailbox");

    assert!(!exported.exported_at.is_empty());
    assert!(exported.exported_at.contains("UTC"));
}

/// Test commit_archive creates a git commit with exported mailbox
#[tokio::test]
async fn test_commit_archive() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let (_, slug) = setup_project_with_messages(&tc, "archive").await;

    let commit_message = "Archive mailbox for testing";
    let oid = ExportBmc::commit_archive(&tc.ctx, &tc.mm, &slug, commit_message)
        .await
        .expect("Failed to commit archive");

    // Verify OID is a valid git hash (40 hex characters)
    assert_eq!(oid.len(), 40, "Git OID should be 40 characters");
    assert!(
        oid.chars().all(|c| c.is_ascii_hexdigit()),
        "Git OID should be hexadecimal"
    );
}

/// Test commit_archive for empty mailbox
#[tokio::test]
async fn test_commit_archive_empty_mailbox() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let human_key = "/test/empty-archive-repo";
    let slug = slugify(human_key);

    ProjectBmc::create(&tc.ctx, &tc.mm, &slug, human_key)
        .await
        .expect("Failed to create project");

    let oid = ExportBmc::commit_archive(&tc.ctx, &tc.mm, &slug, "Archive empty mailbox")
        .await
        .expect("Failed to commit empty archive");

    // Should still create a valid commit even with no messages
    assert_eq!(oid.len(), 40, "Git OID should be 40 characters");
}

/// Test commit_archive for nonexistent project fails
#[tokio::test]
async fn test_commit_archive_nonexistent_project() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    let result =
        ExportBmc::commit_archive(&tc.ctx, &tc.mm, "nonexistent-archive-slug", "Should fail").await;

    assert!(
        result.is_err(),
        "commit_archive should fail for nonexistent project"
    );
}

#[tokio::test]
async fn test_export_signing_keypair_generation() {
    use lib_core::model::export::{
        generate_signing_keypair, signing_key_to_base64, verifying_key_to_base64,
    };

    let (signing_key, verifying_key) = generate_signing_keypair();

    let private_b64 = signing_key_to_base64(&signing_key);
    let public_b64 = verifying_key_to_base64(&verifying_key);

    assert!(!private_b64.is_empty());
    assert!(!public_b64.is_empty());
    assert_ne!(private_b64, public_b64);
}

#[tokio::test]
async fn test_export_manifest_signing() {
    use lib_core::model::export::{ExportManifest, ExportedMailbox, generate_signing_keypair};

    let exported = ExportedMailbox {
        project_slug: "test-project".to_string(),
        project_name: "Test Project".to_string(),
        message_count: 5,
        exported_at: "2025-12-20T00:00:00Z".to_string(),
        content: "test content".to_string(),
        format: "json".to_string(),
    };

    let (signing_key, _) = generate_signing_keypair();
    let mut manifest = ExportManifest::new(&exported);

    assert!(manifest.signature.is_none());
    assert!(manifest.public_key.is_none());

    manifest.sign(&signing_key);

    assert!(manifest.signature.is_some());
    assert!(manifest.public_key.is_some());
}

#[tokio::test]
async fn test_export_manifest_verification() {
    use lib_core::model::export::{ExportManifest, ExportedMailbox, generate_signing_keypair};

    let exported = ExportedMailbox {
        project_slug: "test-project".to_string(),
        project_name: "Test Project".to_string(),
        message_count: 5,
        exported_at: "2025-12-20T00:00:00Z".to_string(),
        content: "test content".to_string(),
        format: "json".to_string(),
    };

    let (signing_key, _) = generate_signing_keypair();
    let mut manifest = ExportManifest::new(&exported);
    manifest.sign(&signing_key);

    let verified = manifest.verify().expect("Verification should succeed");
    assert!(verified, "Signed manifest should verify");
}

#[tokio::test]
async fn test_export_manifest_tamper_detection() {
    use lib_core::model::export::{ExportManifest, ExportedMailbox, generate_signing_keypair};

    let exported = ExportedMailbox {
        project_slug: "test-project".to_string(),
        project_name: "Test Project".to_string(),
        message_count: 5,
        exported_at: "2025-12-20T00:00:00Z".to_string(),
        content: "test content".to_string(),
        format: "json".to_string(),
    };

    let (signing_key, _) = generate_signing_keypair();
    let mut manifest = ExportManifest::new(&exported);
    manifest.sign(&signing_key);

    manifest.message_count = 10;

    let verified = manifest.verify().expect("Verification call should succeed");
    assert!(!verified, "Tampered manifest should not verify");
}

#[tokio::test]
async fn test_export_verify_with_external_key() {
    use lib_core::model::export::{
        ExportManifest, ExportedMailbox, generate_signing_keypair, verifying_key_to_base64,
    };

    let exported = ExportedMailbox {
        project_slug: "test-project".to_string(),
        project_name: "Test Project".to_string(),
        message_count: 5,
        exported_at: "2025-12-20T00:00:00Z".to_string(),
        content: "test content".to_string(),
        format: "json".to_string(),
    };

    let (signing_key, verifying_key) = generate_signing_keypair();
    let public_b64 = verifying_key_to_base64(&verifying_key);

    let mut manifest = ExportManifest::new(&exported);
    manifest.sign(&signing_key);

    let verified = manifest
        .verify_with_key(&public_b64)
        .expect("Verification should succeed");
    assert!(verified, "Should verify with correct public key");
}

#[tokio::test]
async fn test_export_verify_wrong_key_fails() {
    use lib_core::model::export::{
        ExportManifest, ExportedMailbox, generate_signing_keypair, verifying_key_to_base64,
    };

    let exported = ExportedMailbox {
        project_slug: "test-project".to_string(),
        project_name: "Test Project".to_string(),
        message_count: 5,
        exported_at: "2025-12-20T00:00:00Z".to_string(),
        content: "test content".to_string(),
        format: "json".to_string(),
    };

    let (signing_key, _) = generate_signing_keypair();
    let (_, wrong_key) = generate_signing_keypair();
    let wrong_public_b64 = verifying_key_to_base64(&wrong_key);

    let mut manifest = ExportManifest::new(&exported);
    manifest.sign(&signing_key);

    let verified = manifest
        .verify_with_key(&wrong_public_b64)
        .expect("Verification call should succeed");
    assert!(!verified, "Should fail with wrong public key");
}

#[tokio::test]
async fn test_export_mailbox_signed() {
    let tc = TestContext::new()
        .await
        .expect("Failed to create test context");

    use lib_core::model::export::generate_signing_keypair;

    let (_, slug) = setup_project_with_messages(&tc, "signed").await;
    let (signing_key, _) = generate_signing_keypair();

    let (exported, manifest) = ExportBmc::export_mailbox_signed(
        &tc.ctx,
        &tc.mm,
        &slug,
        ExportFormat::Json,
        ScrubMode::None,
        false,
        Some(&signing_key),
    )
    .await
    .expect("Failed to export signed mailbox");

    assert!(manifest.signature.is_some());
    assert!(manifest.public_key.is_some());

    let verified =
        ExportBmc::verify_export(&exported, &manifest).expect("Verification should succeed");
    assert!(verified, "Signed export should verify");
}

#[tokio::test]
async fn test_age_identity_generation() {
    use lib_core::model::export::generate_age_identity;

    let (identity, recipient) = generate_age_identity();

    // Identity should start with "AGE-SECRET-KEY-"
    assert!(
        identity.starts_with("AGE-SECRET-KEY-"),
        "Identity should start with AGE-SECRET-KEY-"
    );

    // Recipient should start with "age1"
    assert!(
        recipient.starts_with("age1"),
        "Recipient should start with age1"
    );

    // Both should be valid strings
    assert!(!identity.is_empty(), "Identity should not be empty");
    assert!(!recipient.is_empty(), "Recipient should not be empty");
}

#[tokio::test]
async fn test_age_encrypt_decrypt_with_passphrase() {
    use lib_core::model::export::{decrypt_with_passphrase, encrypt_with_passphrase};

    let test_data = b"Hello, world! This is a test message for age encryption.";
    let passphrase = "test-passphrase-123";

    // Encrypt
    let encrypted =
        encrypt_with_passphrase(test_data, passphrase).expect("Encryption should succeed");

    // Should be armored (ASCII)
    let encrypted_str = String::from_utf8(encrypted.clone()).expect("Should be valid UTF-8");
    assert!(
        encrypted_str.contains("-----BEGIN AGE ENCRYPTED FILE-----"),
        "Should contain armor header"
    );

    // Decrypt
    let decrypted =
        decrypt_with_passphrase(&encrypted, passphrase).expect("Decryption should succeed");

    // Should match original
    assert_eq!(decrypted, test_data, "Decrypted data should match original");
}

#[tokio::test]
async fn test_age_encrypt_decrypt_with_keypair() {
    use lib_core::model::export::{decrypt_with_identity, encrypt_with_age, generate_age_identity};

    let test_data = b"Hello, world! This is a test message for age key encryption.";
    let (identity, recipient) = generate_age_identity();

    // Encrypt
    let encrypted = encrypt_with_age(test_data, &[recipient]).expect("Encryption should succeed");

    // Should be armored (ASCII)
    let encrypted_str = String::from_utf8(encrypted.clone()).expect("Should be valid UTF-8");
    assert!(
        encrypted_str.contains("-----BEGIN AGE ENCRYPTED FILE-----"),
        "Should contain armor header"
    );

    // Decrypt
    let decrypted =
        decrypt_with_identity(&encrypted, &identity).expect("Decryption should succeed");

    // Should match original
    assert_eq!(decrypted, test_data, "Decrypted data should match original");
}

#[tokio::test]
async fn test_age_wrong_passphrase_fails() {
    use lib_core::model::export::{decrypt_with_passphrase, encrypt_with_passphrase};

    let test_data = b"Secret message";
    let passphrase = "correct-passphrase";
    let wrong_passphrase = "wrong-passphrase";

    // Encrypt
    let encrypted =
        encrypt_with_passphrase(test_data, passphrase).expect("Encryption should succeed");

    // Try to decrypt with wrong passphrase - should fail
    let result = decrypt_with_passphrase(&encrypted, wrong_passphrase);
    assert!(
        result.is_err(),
        "Decryption with wrong passphrase should fail"
    );
}

#[tokio::test]
async fn test_age_wrong_identity_fails() {
    use lib_core::model::export::{decrypt_with_identity, encrypt_with_age, generate_age_identity};

    let test_data = b"Secret message";
    let (identity, recipient) = generate_age_identity();
    let (wrong_identity, _) = generate_age_identity();

    // Encrypt
    let encrypted = encrypt_with_age(test_data, &[recipient]).expect("Encryption should succeed");

    // Try to decrypt with wrong identity - should fail
    let result = decrypt_with_identity(&encrypted, &wrong_identity);
    assert!(
        result.is_err(),
        "Decryption with wrong identity should fail"
    );
}
