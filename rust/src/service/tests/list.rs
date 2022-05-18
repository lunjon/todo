use super::*;
use crate::error::Result;
use crate::model::Status;

#[tokio::test]
async fn list_todos_filter_none() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todos = fixture.service.list_todos(None).await?;
    assert!(!todos.is_empty());
    Ok(())
}

#[tokio::test]
async fn list_todos_filter_status_done() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let filter = Filter::default().status(StatusFilter::Status(Status::Done));
    let todos = fixture.service.list_todos(Some(filter)).await?;
    assert_eq!(todos.len(), 1);
    Ok(())
}

#[tokio::test]
async fn list_todos_with_context_set() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture
        .service
        .set_context(&fixture.test_context_name)
        .await?;
    let expected = fixture
        .service
        .add_todo(
            Status::New,
            Prio::Normal,
            "Title".to_string(),
            "Description".to_string(),
            Tags::default(),
        )
        .await?;

    let filter = Filter::default();
    if let [actual] = &fixture.service.list_todos(Some(filter)).await?[..] {
        assert_eq!(expected.id, actual.id);
    } else {
        panic!();
    }
    Ok(())
}
