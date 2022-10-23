use super::*;
use crate::error::Result;
use crate::model::{Prio, Status};

#[tokio::test]
async fn add_todo() -> Result<()> {
    let fixture = Fixture::setup().await?;
    let todo = fixture
        .svc
        .add_todo(
            Status::New,
            Prio::Normal,
            "Subject".to_string(),
            "description".to_string(),
            CSV::default(),
        )
        .await?;
    assert_eq!(todo.subject, "Subject");
    assert_eq!(todo.status, Status::New);
    assert!(todo.links.is_empty());
    Ok(())
}

// #[tokio::test]
// async fn linking_unknown_gives_error() {
//     todo!()
// }
