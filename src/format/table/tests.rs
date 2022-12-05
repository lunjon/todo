use super::*;
use crate::model::*;
use chrono::Local;

#[test]
fn test_format_todos() {
    let f = TableFormatter::new(true);
    let todos = build_todos();
    let s = f.todos(&todos);
    assert!(!s.is_empty());
}

#[test]
fn test_format_todo() {
    let f = TableFormatter::new(true);
    let todos = build_todos();
    let todo = todos.get(0).unwrap();
    let s = f.todo(todo);
    assert!(!s.is_empty());
}

fn build_todos() -> Vec<Todo> {
    let dt = Local::now();

    vec![
        Todo::new(
            ID::new(1),
            dt,
            Status::New,
            Prio::Low,
            "new|no tags|no context".to_string(),
            "1st".to_string(),
            CSV::default(),
            None,
            CSV::empty(),
        ),
        Todo::new(
            ID::new(2),
            dt,
            Status::New,
            Prio::Normal,
            "new|feat|no context".to_string(),
            "2nd".to_string(),
            CSV::new(vec!["feat".to_string()]),
            None,
            CSV::empty(),
        ),
        Todo::new(
            ID::new(3),
            dt,
            Status::New,
            Prio::High,
            "new|feat,test|no context".to_string(),
            "3rd".to_string(),
            CSV::new(vec!["feat".to_string(), "test".to_string()]),
            None,
            CSV::empty(),
        ),
        Todo::new(
            ID::new(4),
            dt,
            Status::New,
            Prio::Normal,
            "new|no tags|context:home".to_string(),
            "4th".to_string(),
            CSV::default(),
            Some("home".to_string()),
            CSV::empty(),
        ),
        Todo::new(
            ID::new(5),
            dt,
            Status::Done,
            Prio::Critical,
            "done|test|context:home".to_string(),
            "4th".to_string(),
            CSV::new(vec!["test".to_string()]),
            Some("home".to_string()),
            CSV::empty(),
        ),
    ]
}
