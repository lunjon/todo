use super::*;
use crate::error::Result;
use crate::model::{Prio, Status};

#[tokio::test]
async fn add_todo() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let before = fixture.event_count().await?;
    let todo = fixture
        .service
        .add_todo(
            Status::New,
            Prio::Normal,
            "Subject".to_string(),
            "description".to_string(),
            Tags::default(),
        )
        .await?;
    let after = fixture.event_count().await?;
    assert!(after > before);
    assert_eq!(todo.subject, "Subject");
    assert_eq!(todo.status, Status::New);
    Ok(())
}
