use super::*;
use crate::error::Result;
use crate::model::{Status, ID};

#[tokio::test]
async fn update_unknown_id() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let res = fixture
        .svc
        .update_todo(&ID::new(999), Changeset::default())
        .await;
    let error = res.err().unwrap().to_string();
    assert!(error.contains("not found"));
    Ok(())
}

#[tokio::test]
async fn update_status() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todo = fixture
        .svc
        .update_todo(
            &fixture.todo_new.id,
            Changeset::default().with_status(Status::Done),
        )
        .await?;
    assert_eq!(todo.subject, fixture.todo_new.subject);
    assert_eq!(todo.status, Status::Done);
    assert_eq!(todo.description, fixture.todo_new.description);
    Ok(())
}

#[tokio::test]
async fn update_subject() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todo = fixture
        .svc
        .update_todo(
            &fixture.todo_new.id,
            Changeset::default()
                .with_subject("Updated subject".to_string())
                .with_status(Status::Done),
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
        .svc
        .update_todo(
            &fixture.todo_new.id,
            Changeset::default().with_context("test".to_string()),
        )
        .await?;
    assert_eq!(todo.context, Some("test".to_string()));
    Ok(())
}

// Links

#[tokio::test]
async fn add_link() -> Result<()> {
    let fx = Fixture::setup().await?;

    // Act
    let link = Link::Blocks(fx.todo_new.id);
    fx.svc.link(fx.todo_started.id, link).await?;

    // Assert
    let bi_dir = link.bi_directional(fx.todo_started.id).unwrap();
    let todo_new = fx.svc.get_todo(&fx.todo_new.id).await?;
    assert!(todo_new.links.has_any(&[bi_dir]));

    Ok(())
}

#[tokio::test]
async fn linking_unknown_gives_error() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let link = Link::Blocks(ID::new(99));
    let res = fixture.svc.link(fixture.todo_new.id, link).await;
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn linking_self_gives_error() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let link = Link::Blocks(fixture.todo_new.id);
    let res = fixture.svc.link(fixture.todo_new.id, link).await;
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn linking_same_on_other() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;
    let link = Link::Blocks(fixture.todo_new.id);
    fixture.svc.link(fixture.todo_started.id, link).await?;

    // Act
    let link = Link::Blocks(fixture.todo_started.id);
    let res = fixture.svc.link(fixture.todo_new.id, link).await;

    // Assert
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn add_existing_does_nothing() -> Result<()> {
    let fixture = Fixture::setup().await?;

    // Act
    let link = Link::Blocks(fixture.todo_new.id);
    fixture.svc.link(fixture.todo_started.id, link).await?;
    fixture.svc.link(fixture.todo_started.id, link).await?;
    Ok(())
}

#[tokio::test]
async fn add_blocked_by_sets_status() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;

    // Act: new blocked by started
    let link = Link::BlockedBy(fixture.todo_started.id);
    fixture.svc.link(fixture.todo_new.id, link).await?;

    // Assert
    let todo_new = fixture.svc.get_todo(&fixture.todo_new.id).await?;
    assert!(matches!(todo_new.status, Status::Blocked));
    Ok(())
}

#[tokio::test]
async fn add_blocks_sets_status() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;

    // Act: started blocks new
    let link = Link::Blocks(fixture.todo_new.id);
    fixture.svc.link(fixture.todo_started.id, link).await?;

    // Assert
    let todo_new = fixture.svc.get_todo(&fixture.todo_new.id).await?;
    assert!(matches!(todo_new.status, Status::Blocked));
    Ok(())
}

#[tokio::test]
async fn unlink_block_removes_blockedby() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;

    // Act
    let link = Link::Blocks(fixture.todo_blocked.id);
    fixture.svc.unlink(fixture.todo_started.id, link).await?;

    // Assert
    let todo = fixture.svc.get_todo(&fixture.todo_blocked.id).await?;
    assert!(matches!(todo.status, Status::New));
    Ok(())
}

#[tokio::test]
async fn setting_blocking_done_removes_blockedby() -> Result<()> {
    // Arrange
    let fixture = Fixture::setup().await?;

    // Act
    let cs = Changeset::default().with_status(Status::Done);
    fixture
        .svc
        .update_todo(&fixture.todo_started.id, cs)
        .await?;

    // Assert
    let todo = fixture.svc.get_todo(&fixture.todo_blocked.id).await?;
    assert!(matches!(todo.status, Status::New));
    Ok(())
}
