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
    let s = f.todo(&todo);
    assert!(!s.is_empty());
}

#[test]
fn test_format_events() {
    let f = TableFormatter::new(true);
    let events = build_events();
    let s = f.events(&events);
    assert!(!s.is_empty());
}

fn build_events() -> Vec<Event> {
    let mut todos = build_todos();
    let todo_before = todos.pop().unwrap();
    let mut update_status = todo_before.clone();
    update_status.status = Status::Done;

    let mut update_subject = todo_before.clone();
    update_subject.subject = "New subject".to_string();

    vec![
        Event::new(
            ID::new(1),
            Action::Add,
            Kind::AddTodo(todos.pop().unwrap()),
            1_000_000,
        ),
        Event::new(
            ID::new(2),
            Action::Add,
            Kind::RemoveTodo(todos.pop().unwrap()),
            2_000_000,
        ),
        Event::new(
            ID::new(3),
            Action::Add,
            Kind::UpdateTodo {
                before: todo_before.clone(),
                after: update_status,
            },
            3_000_000,
        ),
        Event::new(
            ID::new(4),
            Action::Add,
            Kind::UpdateTodo {
                before: todo_before.clone(),
                after: update_subject,
            },
            4_000_000,
        ),
        Event::new(
            ID::new(5),
            Action::Add,
            Kind::AddContext("home".to_string()),
            5_000_000,
        ),
        Event::new(
            ID::new(5),
            Action::Add,
            Kind::SetContext {
                before: "".to_string(),
                after: "home".to_string(),
            },
            5_000_000,
        ),
        Event::new(
            ID::new(6),
            Action::Add,
            Kind::RemoveContext("home".to_string(), vec![todo_before.clone()]),
            6_000_000,
        ),
    ]
}

fn build_todos() -> Vec<Todo> {
    let dt = Local::now();

    vec![
        Todo::new(
            ID::new(1),
            dt.clone(),
            Status::New,
            Prio::Low,
            "new|no tags|no context".to_string(),
            "1st".to_string(),
            Tags::default(),
            None,
        ),
        Todo::new(
            ID::new(2),
            dt.clone(),
            Status::New,
            Prio::Normal,
            "new|feat|no context".to_string(),
            "2nd".to_string(),
            Tags::new(vec!["feat".to_string()]),
            None,
        ),
        Todo::new(
            ID::new(3),
            dt.clone(),
            Status::New,
            Prio::High,
            "new|feat,test|no context".to_string(),
            "3rd".to_string(),
            Tags::new(vec!["feat".to_string(), "test".to_string()]),
            None,
        ),
        Todo::new(
            ID::new(4),
            dt.clone(),
            Status::New,
            Prio::Normal,
            "new|no tags|context:home".to_string(),
            "4th".to_string(),
            Tags::default(),
            Some("home".to_string()),
        ),
        Todo::new(
            ID::new(5),
            dt.clone(),
            Status::Done,
            Prio::Critical,
            "done|test|context:home".to_string(),
            "4th".to_string(),
            Tags::new(vec!["test".to_string()]),
            Some("home".to_string()),
        ),
    ]
}
