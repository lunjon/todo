use crate::model::Prio;
use clap::{builder::PossibleValuesParser, command, Arg, Command};

const STATUSES: [&str; 5] = ["any", "new", "started", "done", "blocked"];

pub fn build_app() -> Command<'static> {
    command!()
        .about("CLI tool for managing tasks.")
        .long_about(
            "Use --help option to get detailed help.
If no sub-command is used it defaults to listing todos with the
default behaviour of the list command.",
        )
        .arg(
            Arg::new("log")
                .long("log")
                .help("Enables logging output of the specified level.")
                .takes_value(true)
                .value_parser(PossibleValuesParser::new([
                    "debug", "info", "warning", "error",
                ]))
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
        .subcommand(set())
        .subcommand(edit())
        .subcommand(remove())
        .subcommand(context())
        .subcommand(starship())
        .subcommand(prune())
}

fn show() -> Command<'static> {
    Command::new("show")
        .about("Get more details about a todo.")
        .arg(
            Arg::new("id")
                .help("The ID of the todo to get details for.")
                .takes_value(true)
                .required(true),
        )
}

fn list() -> Command<'static> {
    Command::new("list")
        .visible_alias("ls")
        .about("List todos. Defaults to only listing todos with status != done.")
        .long_about(
            "List todos. Defaults to only listing todos with status != done
and in the current context (if set). To get more details about
a todo use the sub-command 'get'.",
        )
        .arg(
            Arg::new("all")
                .long("all")
                .short('a')
                .exclusive(true)
                .help("List all todos.")
                .required(false),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .short('s')
                .help("Filter on status.")
                .takes_value(true)
                .value_parser(PossibleValuesParser::new(STATUSES)),
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
        .visible_alias("new")
        .about("Adds a new todo.")
        .long_about(
            "Adds a new todo. Required parameters not passed via options are queried interactively."
        )
        .arg(
            Arg::new("subject")
                .long("subject")
                .short('s')
                .help("Subject of the todo.")
                .takes_value(true),
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .help("Description of the todo.")
                .takes_value(true),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .multiple_values(true)
                .takes_value(true)
                .help("Tag the todo. Must be a single word. Can be passed multiple times."),
        )
        .arg(
            Arg::new("prio")
                .long("prio")
                .takes_value(true)
                .value_parser(PossibleValuesParser::new([
                    "low", "normal", "high", "cirital",
                ]))
                .help("Sets another priority than the default: normal."),
        )
}

pub fn set() -> Command<'static> {
    Command::new("set")
        .about("Set/update properties of a todo.")
        .arg(
            Arg::new("id")
                .takes_value(true)
                .required(true)
                .help("ID of the todo to update."),
        )
        .arg(
            Arg::new("subject")
                .long("subject")
                .short('t')
                .takes_value(true)
                .help("New subject of the todo."),
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .takes_value(true)
                .help("New description of the todo."),
        )
        .arg(
            Arg::new("status")
                .long("status")
                .short('s')
                .takes_value(true)
                .help("New status of the todo.")
                .value_parser(PossibleValuesParser::new([
                    "new", "started", "done", "blocked",
                ])),
        )
        .arg(
            Arg::new("prio")
                .long("prio")
                .short('p')
                .takes_value(true)
                .help("New priority of the todo.")
                .value_parser(Prio::values()),
        )
        .arg(
            Arg::new("context")
                .long("context")
                .short('c')
                .takes_value(true)
                .help("Sets context of the todo. Use empty string to unset the context."),
        )
        .arg(
            Arg::new("link")
                .long("link")
                .help("Add a link by type:id.")
                .long_help(
                    "Add a link by type:id, like so:
    blocks:id
    blocked-by:id
    relates-to:id

Some links are bi-directional: `a blocks b` implices `b blocked by a`.
",
                )
                .takes_value(true)
                .conflicts_with("unlink"),
        )
        .arg(
            Arg::new("unlink")
                .long("unlink")
                .help("Removes a link by type:id. See --link for valid options.")
                .takes_value(true)
                .conflicts_with("link"),
        )
}

pub fn edit() -> Command<'static> {
    Command::new("edit")
        .about("Edit a todo with an editor.")
        .arg(
            Arg::new("id")
                .takes_value(true)
                .required(true)
                .help("ID of the todo to update."),
        )
        .arg(
            Arg::new("description")
                .long("description")
                .short('d')
                .help("Edit only the description of the todo.")
                .required(false),
        )
}

fn done() -> Command<'static> {
    Command::new("done")
        .about("Marks one or more todos as done.")
        .long_about(
            "Marks one or more todos as done.
To update other fields use the 'update' command.",
        )
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the todos to set as done.")
                .long_help(
                    "IDs of the todos to set as done.
Only valid IDs of type unsigned integers will be considered.",
                ),
        )
}

fn start() -> Command<'static> {
    Command::new("start")
        .about("Set status of one or more todos to started.")
        .long_about(
            "Marks one or more todos as done.
To update other fields use the 'update' command.",
        )
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the todos to remove. Only valid IDs will be considered."),
        )
}

fn remove() -> Command<'static> {
    Command::new("remove")
        .visible_alias("rm")
        .about("Removes one or more todos.")
        .arg(
            Arg::new("ids")
                .multiple_values(true)
                .takes_value(true)
                .help("IDs of the todos to remove. Only valid IDs will be considered."),
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
        .visible_alias("cx")
        .visible_alias("ctx")
        .about("Shows the current context, if any. See sub-commands for managing contexts.")
        .long_about(
            "Contexts are used to associate todos with a certain context,
e.g. 'home' or 'work'. Contexts must have a name with a length
greater than 2 and no more than 10.

Before a context can be referenced, i.e. set, it must be created
via the 'context add' sub-command.",
        )
        .arg(
            Arg::new("add")
                .long("add")
                .short('a')
                .help("Add new context with the given name.")
                .value_name("NAME")
                .exclusive(true),
        )
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .help("List available contexts.")
                .takes_value(false)
                .exclusive(true),
        )
        .arg(
            Arg::new("remove")
                .long("remove")
                .short('r')
                .help("Remove context with the given name.")
                .value_name("NAME")
                .conflicts_with_all(&["add", "list", "set", "unset"])
                .exclusive(false),
        )
        .arg(
            Arg::new("cascade")
                .long("cascade")
                .takes_value(false)
                .requires("remove")
                .help("Remove all todos associated with context."),
        )
        .arg(
            Arg::new("set")
                .long("set")
                .short('s')
                .help("Set context with the given name.")
                .value_name("NAME")
                .exclusive(true),
        )
        .arg(
            Arg::new("unset")
                .long("unset")
                .short('u')
                .help("Unset current context if set.")
                .takes_value(false)
                .exclusive(true),
        )
}

fn starship() -> Command<'static> {
    Command::new("starship")
        .about("Output information for Starship Prompt.")
        .hide(true)
        .arg(Arg::new("when").long("when").takes_value(false))
}

fn prune() -> Command<'static> {
    Command::new("prune").about("Prune todos.").arg(
        Arg::new("done")
            .help("Prune all todos with status 'done'.")
            .long("done")
            .takes_value(false),
    )
}
