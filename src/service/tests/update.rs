use super::*;
use crate::error::Result;
use crate::model::{Status, ID};

#[tokio::test]
async fn update_unknown_id() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let res = fixture
        .service
        .update_todo(&ID::new(999), None, Some(Status::Done), None, None, None)
        .await;
    let error = res.err().unwrap().to_string();
    assert!(error.contains("not found"));
    Ok(())
}

#[tokio::test]
async fn update_status() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let before = fixture.event_count().await?;
    let todo = fixture
        .service
        .update_todo(
            &fixture.todo_new.id,
            None,
            Some(Status::Done),
            None,
            None,
            None,
        )
        .await?;
    let after = fixture.event_count().await?;
    assert!(after > before);

    assert_eq!(todo.subject, fixture.todo_new.subject);
    assert_eq!(todo.status, Status::Done);
    assert_eq!(todo.description, fixture.todo_new.description);
    Ok(())
}

#[tokio::test]
async fn update_subject() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todo = fixture
        .service
        .update_todo(
            &fixture.todo_new.id,
            Some("Updated subject".to_string()),
            Some(Status::Done),
            None,
            None,
            None,
        )
        .await?;
    assert_eq!(todo.subject, "Updated subject");
    assert_eq!(todo.status, Status::Done);
    Ok(())
}

#[tokio::test]
async fn update_context() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todo = fixture
        .service
        .update_todo(
            &fixture.todo_new.id,
            None,
            None,
            None,
            None,
            Some("test".to_string()),
        )
        .await?;
    assert_eq!(todo.context, Some("test".to_string()));
    Ok(())
}
