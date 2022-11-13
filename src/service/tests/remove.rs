use super::*;
use crate::error::{Error, Result};

#[tokio::test]
async fn remove_new_todo() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture.svc.remove_todo(&fixture.todo_new.id).await?;
    assert!(!fixture.todo_exists(&fixture.todo_new.id).await?);
    Ok(())
}

#[tokio::test]
async fn remove_done_todo() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture.svc.remove_todo(&fixture.todo_done.id).await?;
    assert!(!fixture.todo_exists(&fixture.todo_done.id).await?);
    Ok(())
}

#[tokio::test]
async fn remove_unknown() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let err = match fixture.svc.remove_todo(&ID::new(9999)).await {
        Err(Error::NotFound(_)) => true,
        _ => false,
    };
    assert!(err);
    Ok(())
}

#[tokio::test]
async fn removing_todo_removes_links() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;

    // Act
    fixture.svc.remove_todo(&fixture.todo_started.id).await?;

    // Assert
    let todo = fixture.svc.get_todo(&fixture.todo_blocked.id).await?;
    assert!(matches!(todo.status, Status::New));
    Ok(())
}
