use crate::err;
use crate::error::Result;
use crate::format::{Card, Formatter, TableFormatter};
use crate::model::{Link, Prio, Status, CSV, ID};
use crate::service::changeset::Changeset;
use crate::service::{ContextFilter, Filter, PruneFilter, Service, StatusFilter};
use crate::style::{Color, Styler};
use clap::ArgMatches;
use std::path::PathBuf;
use std::process;

mod app;
mod interaction;

use interaction::{Editor, StdinPrompt};

pub struct Cli {
    #[allow(dead_code)]
    root: PathBuf,
    service: Service,
    prompt: StdinPrompt,
    formatter: Box<dyn Formatter>,
    blue_styler: Styler,
    bold_styler: Styler,
    red_styler: Styler,
    green_styler: Styler,
    yellow_styler: Styler,
}

impl Cli {
    pub fn new(root: PathBuf, service: Service) -> Self {
        Self {
            root,
            service,
            prompt: StdinPrompt::default(),
            formatter: Box::new(TableFormatter::new(true)),
            blue_styler: Styler::default().fg(Color::Blue),
            bold_styler: Styler::default().bold(true),
            red_styler: Styler::default().bold(true).fg(Color::Red),
            green_styler: Styler::default().fg(Color::Green),
            yellow_styler: Styler::default().fg(Color::Yellow),
        }
    }

    pub async fn exec(&self) -> Result<()> {
        let matches = app::build_app().get_matches();
        let log_level = matches.get_one::<String>("log");
        self.enable_log(log_level.map(|s| s.as_str()))?;

        match matches.subcommand() {
            None => self.handle_default().await?,
            Some(("show", sub_matches)) => self.handle_get(sub_matches).await?,
            Some(("list", sub_matches)) => self.handle_list(sub_matches).await?,
            Some(("add", sub_matches)) => self.handle_add(sub_matches).await?,
            Some(("remove", sub_matches)) => self.handle_remove(sub_matches).await?,
            Some(("done", sub_matches)) => self.handle_done(sub_matches).await?,
            Some(("start", sub_matches)) => self.handle_start(sub_matches).await?,
            Some(("set", sub_matches)) => self.handle_set(sub_matches).await?,
            Some(("edit", sub_matches)) => self.handle_edit(sub_matches).await?,
            Some(("context", sub_matches)) => self.handle_context(sub_matches).await?,
            Some(("starship", sub_matches)) => self.handle_starship(sub_matches).await?,
            Some(("prune", sub_matches)) => self.handle_prune(sub_matches).await?,
            _ => unreachable!(),
        }

        Ok(())
    }

    fn enable_log(&self, value: Option<&str>) -> Result<()> {
        let level = match value {
            Some(level) => level,
            None => return Ok(()),
        };

        let level = match level.trim() {
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" | "warning" => log::LevelFilter::Warn,
            "err" | "error" => log::LevelFilter::Error,
            _ => return err!("invalid log level: {}", level),
        };

        env_logger::builder().filter(Some("todo"), level).init();
        Ok(())
    }
}

impl Cli {
    // Only list todos with default filter. No options supported.
    async fn handle_default(&self) -> Result<()> {
        let todos = self.service.list_todos(Some(Filter::default())).await?;
        if !todos.is_empty() {
            println!("{}", self.formatter.todos(&todos));
        }
        Ok(())
    }

    async fn handle_get(&self, matches: &ArgMatches) -> Result<()> {
        let id = Self::parse_id(matches.get_one::<String>("id").unwrap().as_str())?;
        let todo = self.service.get_todo(&id).await?;
        let card = Card::new(true);
        let s = card.format(&todo);
        println!("{s}");
        Ok(())
    }

    async fn handle_list(&self, matches: &ArgMatches) -> Result<()> {
        let filter = if matches.contains_id("all") {
            None
        } else {
            let filter = Filter::default();

            let filter = match matches.get_one::<String>("status") {
                Some(status) => match status.as_str() {
                    "any" => filter.status(StatusFilter::Any),
                    status => {
                        let s = Status::try_from(status.to_string())?;
                        filter.status(StatusFilter::Status(s))
                    }
                },
                None => filter,
            };

            let filter = match matches.get_one::<String>("context") {
                Some(s) => filter.context(ContextFilter::Name(s.to_string())),
                None => filter,
            };

            let filter = match matches.get_many::<String>("tags") {
                Some(tags) => filter.tags(tags.map(String::from).collect()),
                None => filter,
            };

            Some(filter)
        };

        let todos = self.service.list_todos(filter).await?;
        if !todos.is_empty() {
            println!("{}", self.formatter.todos(&todos));
        }
        Ok(())
    }

    async fn handle_add(&self, matches: &ArgMatches) -> Result<()> {
        let subject = match matches.get_one::<String>("subject") {
            Some(s) => s.to_string(),
            None => self.prompt.line("subject>", false)?,
        };
        log::debug!("New todo: subject={}", subject);

        let prio = match matches.get_one::<String>("prio") {
            Some(s) => Prio::try_from(s.to_string())?,
            None => match self
                .prompt
                .select(
                    "priority",
                    vec![
                        self.bold_styler.style("normal"),
                        self.blue_styler.style("low"),
                        self.yellow_styler.style("high"),
                        self.red_styler.style("critical"),
                    ],
                )?
                .as_str()
            {
                "" => Prio::Normal,
                s => Prio::try_from(s.to_string())?,
            },
        };
        log::info!("New todo: priority={}", &prio);

        let description = self.get_description(matches)?;
        log::info!("New todo: description={}", &prio);

        let tags: Vec<String> = match matches.get_many::<String>("tag") {
            Some(s) => s.map(String::from).collect(),
            None => match self
                .prompt
                .line("tags (single words, comma separated)", true)?
                .trim()
            {
                "" => vec![],
                s => s.split(',').map(|s| s.trim().to_string()).collect(),
            },
        };
        log::debug!("New todo: tags={:?}", tags);

        let todo = self
            .service
            .add_todo(Status::New, prio, subject, description, CSV::new(tags))
            .await?;

        println!("{}", self.formatter.todo(&todo));

        Ok(())
    }

    async fn handle_done(&self, matches: &ArgMatches) -> Result<()> {
        let ids = Self::get_ids(matches)?;
        let mut updated = Vec::new();

        for id in ids {
            let changeset = Changeset::default().with_status(Status::Done);
            let todo = self.service.update_todo(&id, changeset).await?;
            updated.push(todo);
        }

        if !updated.is_empty() {
            println!("{}", self.formatter.todos(&updated));
        }
        Ok(())
    }

    async fn handle_start(&self, matches: &ArgMatches) -> Result<()> {
        let ids = Self::get_ids(matches)?;
        let mut updated = Vec::new();

        for id in ids {
            let changeset = Changeset::default().with_status(Status::Started);
            let todo = self.service.update_todo(&id, changeset).await?;
            updated.push(todo);
        }

        if !updated.is_empty() {
            println!("{}", self.formatter.todos(&updated));
        }
        Ok(())
    }

    async fn handle_set(&self, matches: &ArgMatches) -> Result<()> {
        let id = Self::parse_id(matches.get_one::<String>("id").unwrap().as_str())?;

        let changeset = Changeset::default();
        let changeset = match matches.get_one::<String>("subject") {
            Some(s) => changeset.with_subject(s.to_string()),
            None => changeset,
        };

        let changeset = match matches.get_one::<String>("status") {
            Some(value) => changeset.with_status(Status::try_from(value.to_string())?),
            None => changeset,
        };

        let changeset = match matches.get_one::<String>("prio") {
            Some(s) => changeset.with_prio(Prio::try_from(s.to_string())?),
            None => changeset,
        };

        let changeset = match matches.get_one::<String>("description") {
            Some(s) => changeset.with_description(s.to_string()),
            None => changeset,
        };

        let changeset = match matches.get_one::<String>("context") {
            Some(s) => changeset.with_context(s.to_string()),
            None => changeset,
        };

        let todo = self.service.update_todo(&id, changeset).await?;

        // Linking requires additional rules and validation
        if let Some(links) = matches.get_many::<String>("link") {
            for link in links {
                self.service
                    .link(todo.id, Link::try_from(link.as_str())?)
                    .await?;
            }
        }
        if let Some(links) = matches.get_many::<String>("unlink") {
            for link in links {
                self.service
                    .unlink(todo.id, Link::try_from(link.as_str())?)
                    .await?;
            }
        }

        println!("{}", self.formatter.todo(&todo));
        Ok(())
    }

    async fn handle_edit(&self, matches: &ArgMatches) -> Result<()> {
        let id = Self::parse_id(matches.get_one::<String>("id").unwrap().as_str())?;
        log::info!("Updating todo with id {} from editor", id);

        let todo = self.service.get_todo(&id).await?;
        let changeset = if matches.contains_id("description") {
            log::debug!("Only editing description of the todo");
            let desc = Editor::string(&todo.description)?;
            Changeset::default().with_description(desc)
        } else {
            log::debug!("Editing the whole todo");
            Editor::todo(&todo)?
        };

        let todo = self.service.update_todo(&id, changeset).await?;
        println!("{}", self.formatter.todo(&todo));

        Ok(())
    }

    async fn handle_context(&self, matches: &ArgMatches) -> Result<()> {
        if let Some(cx) = matches.get_one::<String>("add") {
            self.service.add_context(cx).await?;
            println!(
                "Added new context with name {}.",
                self.green_styler.style(cx)
            );

            if self.prompt.confirm("Activate new context?", false)? {
                self.service.set_context(cx).await?;
                println!("Context set to {}.", self.green_styler.style(cx));
            }
        } else if let Some(cx) = matches.get_one::<String>("set") {
            self.service.set_context(cx).await?;
            println!("Context set to {}.", self.green_styler.style(cx));
        } else if let Some(cx) = matches.get_one::<String>("remove") {
            let cascade = matches.contains_id("cascade");
            self.service.remove_context(cx, cascade).await?;
            println!("Removed context {}", cx);
        } else if matches.contains_id("unset") {
            self.service.unset_context().await?;
            println!("Current context unset.");
        } else if matches.contains_id("list") {
            let contexts = self.service.list_contexts().await?;
            if contexts.is_empty() {
                println!("No contexts created.");
            } else {
                for ctx in contexts {
                    println!("{ctx}");
                }
            }
        } else {
            match self.service.get_context().await? {
                Some(cx) => println!("Context currently set to {}.", self.green_styler.style(&cx)),
                None => println!("No context currently set."),
            }
        }

        Ok(())
    }

    async fn handle_remove(&self, matches: &ArgMatches) -> Result<()> {
        let yes = matches.contains_id("yes");
        let ids = Self::get_ids(matches)?;

        for id in ids {
            if yes {
                self.service.remove_todo(&id).await?;
            } else {
                let msg = format!(
                    "{} todo with ID {}",
                    self.red_styler.style("Remove"),
                    self.green_styler.style(&id.to_string())
                );
                if self.prompt.confirm(&msg, false)? {
                    self.service.remove_todo(&id).await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_starship(&self, matches: &ArgMatches) -> Result<()> {
        let todos = self.service.list_todos(Some(Filter::default())).await?;

        if matches.contains_id("when") {
            if todos.is_empty() {
                process::exit(1);
            } else {
                process::exit(0);
            }
        }

        println!("todo: {}", todos.len());
        Ok(())
    }

    async fn handle_prune(&self, matches: &ArgMatches) -> Result<()> {
        let filter = PruneFilter::default().with_done(matches.contains_id("done"));
        self.service.prune(filter).await
    }

    fn get_description(&self, matches: &ArgMatches) -> Result<String> {
        if let Some(s) = matches.get_one::<String>("description") {
            log::info!("Using description from flag");
            return Ok(s.to_string());
        }

        let options = vec![
            "Open editor".to_string(),
            "Prompt".to_string(),
            "Skip".to_string(),
        ];

        let option = self.prompt.select("Description", options)?;
        match option.as_str() {
            "Open editor" => Editor::empty().edit(),
            "Prompt" => self.prompt.line("Enter description", true),
            "Skip" => Ok(String::new()),
            _ => unreachable!(),
        }
    }

    fn get_ids(matches: &ArgMatches) -> Result<Vec<ID>> {
        let ids = match matches.get_many::<String>("ids") {
            Some(ids) => ids,
            None => return err!("no IDs provided"),
        };

        Ok(ids
            .filter_map(|s| match s.parse::<u16>() {
                Ok(n) => Some(ID::new(n)),
                Err(_) => None,
            })
            .collect())
    }

    fn parse_id(id: &str) -> Result<ID> {
        match id.parse::<u16>() {
            Ok(n) => Ok(ID::new(n)),
            Err(_) => err!("invalid ID format: {}", id),
        }
    }
}
