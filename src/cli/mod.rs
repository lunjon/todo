use crate::error::{Error, Result};
use crate::format::{Card, Formatter, TableFormatter};
use crate::model::{Prio, Status, Tags, ID};
use crate::service::{ContextFilter, Filter, Service, StatusFilter};
use crate::style::{Color, Styler};
use clap::ArgMatches;
use std::path::PathBuf;

mod app;
mod prompt;

use prompt::StdinPrompt;

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
        self.enable_log(matches.value_of("log"))?;

        match matches.subcommand() {
            None => self.handle_default().await?,
            Some(("show", sub_matches)) => self.handle_get(sub_matches).await?,
            Some(("list", sub_matches)) => self.handle_list(sub_matches).await?,
            Some(("add", sub_matches)) => self.handle_add(sub_matches).await?,
            Some(("remove", sub_matches)) => self.handle_remove(sub_matches).await?,
            Some(("done", sub_matches)) => self.handle_done(sub_matches).await?,
            Some(("start", sub_matches)) => self.handle_start(sub_matches).await?,
            Some(("update", sub_matches)) => self.handle_update(sub_matches).await?,
            Some(("context", sub_matches)) => self.handle_context(sub_matches).await?,
            Some(("history", sub_matches)) => self.handle_history(sub_matches).await?,
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
            _ => return Err(Error::ArgError(format!("invalid log level: {}", level))),
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
        let id = Self::parse_id(matches.value_of("id").unwrap())?;
        let todo = self.service.get_todo(&id).await?;
        let card = Card::new(true);
        let s = card.format(&todo);
        println!("{s}");
        Ok(())
    }

    async fn handle_list(&self, matches: &ArgMatches) -> Result<()> {
        let filter = if matches.is_present("all") {
            None
        } else {
            let filter = Filter::default();

            let filter = match matches.value_of("status") {
                Some(status) => match status {
                    "any" => filter.status(StatusFilter::Any),
                    status => {
                        let s = Status::try_from(status.to_string())?;
                        filter.status(StatusFilter::Status(s))
                    }
                },
                None => filter,
            };

            let filter = match matches.value_of("context") {
                Some(s) => filter.context(ContextFilter::Name(s.to_string())),
                None => filter,
            };

            let filter = match matches.values_of("tags") {
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
        let title = match matches.value_of("title") {
            Some(s) => s.to_string(),
            None => self.prompt.line("title>", false)?,
        };

        let prio = match matches.value_of("prio") {
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
        log::info!("Priority: {}", &prio);

        let description = self.get_description(matches)?;

        let tags = match matches.values_of("tags") {
            Some(s) => s.map(|s| s.to_string()).collect::<Vec<String>>().join(" "),
            None => self.prompt.line("tags?", true)?,
        };
        let tags = match Tags::try_from(tags) {
            Ok(tags) => tags,
            Err(err) => return Err(Error::ArgError(err.to_string())),
        };

        let todo = self
            .service
            .add_todo(Status::New, prio, title, description, tags)
            .await?;
        println!("{}", self.formatter.todo(&todo));

        Ok(())
    }

    async fn handle_done(&self, matches: &ArgMatches) -> Result<()> {
        let ids = Self::get_ids(matches)?;
        let mut updated = Vec::new();

        for id in ids {
            let todo = self
                .service
                .update_todo(&id, None, Some(Status::Done), None, None, None)
                .await?;
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
            let todo = self
                .service
                .update_todo(&id, None, Some(Status::Started), None, None, None)
                .await?;
            updated.push(todo);
        }

        if !updated.is_empty() {
            println!("{}", self.formatter.todos(&updated));
        }
        Ok(())
    }

    async fn handle_update(&self, matches: &ArgMatches) -> Result<()> {
        let id = Self::parse_id(matches.value_of("id").unwrap())?;
        let title = matches.value_of("title").map(|s| s.to_string());
        let status = match matches.value_of("status") {
            Some(value) => Some(Status::try_from(value.to_string())?),
            None => None,
        };

        let prio = match matches.value_of("prio") {
            Some(s) => Some(Prio::try_from(s.to_string())?),
            None => None,
        };
        let description = matches.value_of("description").map(|s| s.to_string());
        let context = matches.value_of("context").map(|s| s.to_string());

        let todo = self
            .service
            .update_todo(&id, title, status, prio, description, context)
            .await?;
        println!("{}", self.formatter.todo(&todo));
        Ok(())
    }

    async fn handle_context(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            None => match self.service.get_context().await? {
                Some(cx) => println!("Context currently set to {}.", self.green_styler.style(&cx)),
                None => println!("No context currently set."),
            },
            Some(("add", sub_matches)) => {
                let cx = match sub_matches.value_of("name") {
                    Some(n) => n.to_string(),
                    None => self.prompt.line("Name", false)?,
                };

                self.service.add_context(&cx).await?;
                println!(
                    "Added new context with name {}.",
                    self.green_styler.style(&cx)
                );

                if self.prompt.confirm("Activate new context?", false)? {
                    self.service.set_context(&cx).await?;
                    println!("Context set to {}.", self.green_styler.style(&cx));
                }
            }
            Some(("set", sub_matches)) => {
                let cx = match sub_matches.value_of("name") {
                    Some(name) => name.to_string(),
                    None => {
                        if let Some(cx) = self.select_context().await? {
                            cx
                        } else {
                            return Ok(());
                        }
                    }
                };

                self.service.set_context(&cx).await?;
                println!("Context set to {}.", self.green_styler.style(&cx));
            }
            Some(("unset", _)) => {
                self.service.unset_context().await?;
                println!("Current context unset.");
            }
            Some(("list", _)) => {
                let contexts = self.service.list_contexts().await?;
                if contexts.is_empty() {
                    println!("No contexts created.");
                } else {
                    for ctx in contexts {
                        println!("{ctx}");
                    }
                }
            }
            Some(("remove", sub_matches)) => {
                let cx = match sub_matches.value_of("name") {
                    Some(name) => name.to_string(),
                    None => {
                        if let Some(cx) = self.select_context().await? {
                            cx
                        } else {
                            return Ok(());
                        }
                    }
                };
                let cascade = sub_matches.is_present("cascade");

                self.service.remove_context(&cx, cascade).await?;
                println!("Removed context {}", cx);
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    async fn handle_history(&self, _matches: &ArgMatches) -> Result<()> {
        let events = self.service.list_events().await?;
        if events.is_empty() {
            println!("No history of events.");
        } else {
            println!("{}", self.formatter.events(&events));
        }
        Ok(())
    }

    async fn handle_remove(&self, matches: &ArgMatches) -> Result<()> {
        let yes = matches.is_present("yes");
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

    fn get_description(&self, matches: &ArgMatches) -> Result<String> {
        if let Some(s) = matches.value_of("description") {
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
            "Open editor" => self.prompt.editor(),
            "Prompt" => self.prompt.line("Enter description", true),
            "Skip" => Ok(String::new()),
            _ => unreachable!(),
        }
    }

    async fn select_context(&self) -> Result<Option<String>> {
        let contexts = self.service.list_contexts().await?;
        if contexts.is_empty() {
            println!("No contexts created.");
            return Ok(None);
        }

        let cx = self.prompt.select("Select context", contexts)?;
        Ok(Some(cx))
    }

    fn get_ids(matches: &ArgMatches) -> Result<Vec<ID>> {
        let ids = match matches.values_of("ids") {
            Some(ids) => ids,
            None => return Err(Error::ArgError("no IDs provided".to_string())),
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
            Err(_) => Err(Error::ArgError(format!("invalid ID format: {id}"))),
        }
    }
}
