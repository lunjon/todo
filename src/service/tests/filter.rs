use super::{ContextFilter, Filter, StatusFilter};
use crate::model::{Prio, Status, Tags, Todo, ID};

#[test]
fn test_filter_default() {
    let todos = build_todos();
    let filter = Filter::default();
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 4);
}

#[test]
fn test_filter_context_any() {
    let todos = build_todos();
    let filter = Filter::default().context(ContextFilter::Any);
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 4);
}

#[test]
fn test_filter_context_current_none() {
    let todos = build_todos();
    let filter = Filter::default().context(ContextFilter::Current);
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 4);
}

#[test]
fn test_filter_context_current() {
    let todos = build_todos();
    let filter = Filter::default()
        .context(ContextFilter::Current)
        .status(StatusFilter::Any);
    let todos = filter.apply(todos, Some("home".to_string()));
    assert_eq!(todos.len(), 2);
}

#[test]
fn test_filter_context_name() {
    let todos = build_todos();
    let filter = Filter::default().context(ContextFilter::Name("unknown".to_string()));
    let todos = filter.apply(todos, None);
    assert!(todos.is_empty());
}

#[test]
fn test_filter_status_done() {
    let todos = build_todos();
    let filter = Filter::default().status(StatusFilter::Status(Status::Done));
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 1);
}

#[test]
fn test_filter_status_any() {
    let todos = build_todos();
    let filter = Filter::default().status(StatusFilter::Any);
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 5);
}

#[test]
fn test_filter_status_new_context_current() {
    let todos = build_todos();
    let filter = Filter::default().status(StatusFilter::Status(Status::New));
    let todos = filter.apply(todos, Some("home".to_string()));
    assert_eq!(todos.len(), 1);
}

#[test]
fn test_filter_status_any_context_current() {
    let todos = build_todos();
    let filter = Filter::default().status(StatusFilter::Any);
    let todos = filter.apply(todos, Some("home".to_string()));
    assert_eq!(todos.len(), 2);
}

#[test]
fn test_filter_tags() {
    let todos = build_todos();
    let filter = Filter::default().tags(vec!["feat".to_string()]);
    let todos = filter.apply(todos, None);
    assert_eq!(todos.len(), 2);
}

fn build_todos() -> Vec<Todo> {
    let now = chrono::Local::now();
    vec![
        Todo::new(
            ID::new(1),
            now.clone(),
            Status::New,
            Prio::Normal,
            "new|no tags|no context".to_string(),
            "1st".to_string(),
            Tags::default(),
            None,
        ),
        Todo::new(
            ID::new(2),
            now.clone(),
            Status::New,
            Prio::Normal,
            "new|feat|no context".to_string(),
            "2nd".to_string(),
            Tags::new(vec!["feat".to_string()]),
            None,
        ),
        Todo::new(
            ID::new(3),
            now.clone(),
            Status::New,
            Prio::Normal,
            "new|feat,test|no context".to_string(),
            "3rd".to_string(),
            Tags::new(vec!["feat".to_string(), "test".to_string()]),
            None,
        ),
        Todo::new(
            ID::new(4),
            now.clone(),
            Status::New,
            Prio::Normal,
            "new|no tags|context:home".to_string(),
            "4th".to_string(),
            Tags::default(),
            Some("home".to_string()),
        ),
        Todo::new(
            ID::new(5),
            now.clone(),
            Status::Done,
            Prio::Normal,
            "done|test|context:home".to_string(),
            "4th".to_string(),
            Tags::new(vec!["test".to_string()]),
            Some("home".to_string()),
        ),
    ]
}