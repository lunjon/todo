use super::*;
use crate::error::Result;
use crate::model::Status;

#[tokio::test]
async fn list_todos_filter_none() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todos = fixture.svc.list_todos(None).await?;
    assert!(!todos.is_empty());
    Ok(())
}

#[tokio::test]
async fn list_todos_filter_status_done() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let filter = Filter::default().status(StatusFilter::Status(Status::Done));
    let todos = fixture.svc.list_todos(Some(filter)).await?;
    assert_eq!(todos.len(), 1);
    Ok(())
}

#[tokio::test]
async fn list_todos_with_context_set() -> Result<()> {
    let fixture = Fixture::setup().await?;
    fixture.svc.set_context(&fixture.ctx).await?;
    let expected = fixture
        .svc
        .add_todo(
            Status::New,
            Prio::Normal,
            "Subject".to_string(),
            "Description".to_string(),
            CSV::default(),
        )
        .await?;

    let filter = Filter::default();
    if let [actual] = &fixture.svc.list_todos(Some(filter)).await?[..] {
        assert_eq!(expected.id, actual.id);
    } else {
        panic!();
    }
    Ok(())
}
