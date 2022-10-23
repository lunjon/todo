use clap::{command, Arg, Command};

pub fn build_app() -> Command<'static> {
    command!()
        .about("CLI tool for managing tasks.")
        .long_about(
            "Use --help option to get detailed help.
If no sub-command is used it defaults to listing TODOs with the
default behaviour of the list command.",
        )
        .arg(
            Arg::new("log")
                .long("log")
                .help("Enables logging output of the specified level.")
                .takes_value(true)
                .possible_values(&["debug", "info", "warning", "error"])
                .global(true),
        )
        .arg(
            Arg::new("no-colors")
                .help("Do not display colors in output")
                .long("no-colors")
                .required(false),
        )
        .subcommand(show())
        .subcommand(list())
        .subcommand(add())
        .subcommand(done())
        .subcommand(start())
        .subcommand(update())
        .subcommand(remove())
        .subcommand(context())
        .subcommand(
            Command::new("history")
                .alias("hist")
                .about("Shows an history of events for TODOs."),
        )
}

fn show() -> Command<'static> {
    Command::new("show")
        .about("Get more details about a TODO.")
        .arg(
            Arg::new("id")
                .help("The ID of the TODO to get details for.")
                .takes_value(true)
                .required(true),
        )
}

fn list() -> Command<'static> {
    Command::new("list")
        .alias("ls")
        .about("List TODOs. Defaults to only listing TODOs with status != done.")
        .long_about(
            "List TODOs. Defaults to only listing TODOs with status != done
and in the current context (if set). To get more details about
a TODO use the sub-command 'get'.",
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .exclusive(true)
                .help("List all TODOs.")
                .required(false),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .short('s')
                .help("Filter on status.")
                .takes_value(true)
                .possible_values(&["any", "new", "started", "done"]),
        )
        .arg(
            Arg::new("context")
                .long("context")
                .help("Filter on context. Defaults to only those in current context.")
                .takes_value(true),
        )
        .arg(
            Arg::new("tags")
                .long("tags")
                .multiple_values(true)
                .takes_value(true)
                .help("Filter on tags. Any matching tag is considered a match."),
        )
}

pub fn add() -> Command<'static> {
    Command::new("add")
        .about("Adds a new TODO.")
        .long_about(
            "Adds a new TODO. Required parameters not passed via options are
queried interactively like so:
  > required
  ? optional",
        )
        .arg(
            Arg::new("subject")
                .long("subject")
                .short('t')
                .help("Subject for the TODO.")
                .takes_value(true),
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .help("Description")
                .takes_value(true),
        )
        .arg(
            Arg::new("tags")
                .long("tags")
                .multiple_values(true)
                .takes_value(true)
                .help("Tags for the TODO. Must be single word strings with a length less than 20."),
        )
        .arg(
            Arg::new("prio")
                .long("prio")
                .takes_value(true)
                .possible_values(&["low", "normal", "high", "cirital"])
                .help("Sets another priority than the default: normal."),
        )
}

pub fn update() -> Command<'static> {
    Command::new("update")
        .about("Updates a TODO. Only updates fields provided in the options.")
        .arg(
            Arg::new("id")
                .takes_value(true)
                .required(true)
                .help("ID of the TODO to update."),
        )
        .arg(
            Arg::new("subject")
                .long("subject")
                .short('t')
                .takes_value(true)
                .help("New subject of the TODO."),
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .takes_value(true)
                .help("New description of the TODO."),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .short('s')
                .takes_value(true)
                .help("New status of the TODO.")
                .possible_values(&["new", "started", "done"]),
        )
        .arg(
            Arg::new("prio")
                .long("prio")
                .short('p')
                .takes_value(true)
                .help("New priority of the TODO.")
                .possible_values(&["low", "normal", "high", "critical"]),
        )
        .arg(
            Arg::new("context")
                .long("context")
                .short('c')
                .takes_value(true)
                .help("Sets context of the TODO."),
        )
}

fn done() -> Command<'static> {
    Command::new("done")
        .about("Marks one or more TODOs as done.")
        .long_about(
            "Marks one or more TODOs as done.
To update other fields use the 'update' command.",
        )
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the TODOs to set as done.")
                .long_help(
                    "IDs of the TODOs to set as done.
Only valid IDs of type unsigned integers will be considered.",
                ),
        )
}

fn start() -> Command<'static> {
    Command::new("start")
        .alias("begin")
        .about("Set status of one or more TODOs to started.")
        .long_about(
            "Marks one or more TODOs as done.
To update other fields use the 'update' command.",
        )
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the TODOs to remove. Only valid IDs will be considered."),
        )
}

fn remove() -> Command<'static> {
    Command::new("remove")
        .alias("rm")
        .about("Removes one or more TODOs.")
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the TODOs to remove. Only valid IDs will be considered."),
        )
        .arg(
            Arg::new("yes")
                .long("yes")
                .short('y')
                .help("Do not confirm.")
                .required(false),
        )
}

fn context() -> Command<'static> {
    Command::new("context")
        .alias("ctx")
        .about("Shows the current context, if any. See sub-commands for managing contexts.")
        .long_about(
            "Contexts are used to associate TODOs with a certain context,
e.g. 'home' or 'work'. Contexts must have a name with a length
greater than 2 and no more than 10.

Before a context can be referenced, i.e. set, it must be created
via the 'context add' sub-command.",
        )
        .subcommand(
            Command::new("add").about("Adds a new context.").arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .takes_value(true)
                    .help("Name of the new context."),
            ),
        )
        .subcommand(
            Command::new("set").about("Set context.").arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .takes_value(true)
                    .help("Name of the context to set."),
            ),
        )
        .subcommand(Command::new("unset").about("Unset current context, in any."))
        .subcommand(
            Command::new("remove")
                .alias("rm")
                .about("Removes one or more contexts")
                .arg(
                    Arg::new("cascade")
                        .long("cascade")
                        .takes_value(false)
                        .help("Remove all TODOs associated with context."),
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .takes_value(true)
                        .help("Name of the contexts to remove."),
                ),
        )
        .subcommand(Command::new("list").about("Lists all contexts that have been created."))
}
