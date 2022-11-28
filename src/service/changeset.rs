use crate::model::{Link, Prio, Status, Todo, CSV};

#[derive(Default)]
pub struct Changeset {
    pub status: Option<Status>,
    subject: Option<String>,
    prio: Option<Prio>,
    description: Option<String>,
    context: Option<String>,
    links: Option<CSV<Link>>,
    tags: Option<CSV<String>>,
    updated: bool,
}

impl Changeset {
    pub fn is_empty(&self) -> bool {
        !self.updated
    }

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
        if let Some(s) = self.tags {
            todo.tags = s;
        }
        if let Some(s) = self.links {
            todo.links = s;
        }
    }

    pub fn with_subject(mut self, sub: String) -> Self {
        self.updated = true;
        self.subject = Some(sub);
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.updated = true;
        self.status = Some(status);
        self
    }

    pub fn with_prio(mut self, prio: Prio) -> Self {
        self.updated = true;
        self.prio = Some(prio);
        self
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.updated = true;
        self.description = Some(desc);
        self
    }

    pub fn with_context(mut self, cx: String) -> Self {
        self.updated = true;
        self.context = Some(cx);
        self
    }

    pub fn with_tags(mut self, tags: CSV<String>) -> Self {
        self.updated = true;
        self.tags = Some(tags);
        self
    }

    pub fn with_links(mut self, links: CSV<Link>) -> Self {
        self.updated = true;
        self.links = Some(links);
        self
    }
}
