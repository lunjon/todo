use super::*;
use crate::error::Result;

#[tokio::test]
async fn get_context() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let context = fixture.service.get_context().await?;
    assert!(context.is_none());
    Ok(())
}

#[tokio::test]
async fn get_contexts() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let contexts = fixture.service.list_contexts().await?;
    assert_eq!(contexts.len(), 1);
    Ok(())
}

#[tokio::test]
async fn add_context() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture.service.add_context("new").await?;
    let contexts = fixture.service.list_contexts().await?;
    assert_eq!(contexts.len(), 2);
    Ok(())
}

#[tokio::test]
async fn add_context_invalid_name() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let res = fixture.service.add_context("  ").await;
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn set_known_context() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture
        .service
        .set_context(&fixture.test_context_name)
        .await?;
    let context = fixture.service.get_context().await?;
    assert_eq!(context, Some(fixture.test_context_name));
    Ok(())
}

#[tokio::test]
async fn set_unknown_context() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let res = fixture.service.set_context("unknown").await;
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn add_todo_with_context_set() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture
        .service
        .set_context(&fixture.test_context_name)
        .await?;
    let todo = fixture
        .service
        .add_todo(
            Status::New,
            Prio::Normal,
            "Subject".to_string(),
            "Description".to_string(),
            CSV::default(),
        )
        .await?;
    assert_eq!(todo.context, Some(fixture.test_context_name));
    Ok(())
}

#[tokio::test]
async fn remove_context_no_cascade() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture
        .service
        .set_context(&fixture.test_context_name)
        .await?;
    fixture.create_todo().await?;
    fixture.create_todo().await?;
    let before = fixture.todo_count().await?;
    fixture
        .service
        .remove_context(&fixture.test_context_name, false)
        .await?;

    let after = fixture.todo_count().await?;
    assert_eq!(before, after);
    let context = fixture.service.get_context().await?;
    assert!(context.is_none());
    Ok(())
}

#[tokio::test]
async fn remove_context_cascade() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture
        .service
        .set_context(&fixture.test_context_name)
        .await?;
    fixture.create_todo().await?;
    fixture.create_todo().await?;
    let before = fixture.todo_count().await?;
    fixture
        .service
        .remove_context(&fixture.test_context_name, true)
        .await?;

    let after = fixture.todo_count().await?;
    assert_eq!(after, before - 2);
    let context = fixture.service.get_context().await?;
    assert!(context.is_none());
    Ok(())
}
