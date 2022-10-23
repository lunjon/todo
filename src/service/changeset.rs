use crate::model::{Link, Prio, Status, Todo, CSV};

#[derive(Default)]
pub struct Changeset {
    subject: Option<String>,
    status: Option<Status>,
    prio: Option<Prio>,
    description: Option<String>,
    context: Option<String>,
    links: Option<CSV<Link>>,
}

impl Changeset {
    pub fn apply(self, todo: &mut Todo) {
        if let Some(s) = self.subject {
            todo.subject = s;
        }
        if let Some(s) = self.status {
            todo.status = s;
        }
        if let Some(s) = self.prio {
            todo.prio = s;
        }
        if let Some(s) = self.description {
            todo.description = s;
        }
        if let Some(s) = self.context {
            todo.context = Some(s);
        }
        if let Some(s) = self.links {
            todo.links = s;
        }
    }

    pub fn with_subject(mut self, sub: String) -> Self {
        self.subject = Some(sub);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn with_prio(mut self, prio: Prio) -> Self {
        self.prio = Some(prio);
        self
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    pub fn with_context(mut self, cx: String) -> Self {
        self.context = Some(cx);
        self
    }

    pub fn with_links(mut self, links: CSV<Link>) -> Self {
        self.links = Some(links);
        self
    }
}
