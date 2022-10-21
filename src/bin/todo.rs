use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::path::PathBuf;
use std::str::FromStr;
use todo::cli::Cli;
use todo::err;
use todo::error::Result;
use todo::repository::Repository;
use todo::service::Service;
use todo::style::{Color, Styler};

#[tokio::main]
async fn main() -> Result<()> {
    // Init configuration
    let root = init()?;
    let database_uri = match root.to_str() {
        Some(root) => format!("{root}/todo.db"),
        None => return err!("invalid root path: {:?}", root),
    };

    // Setup database connection and run migrations
    let connection_options = SqliteConnectOptions::from_str(&database_uri)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .read_only(false);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connection_options)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let repository = Repository::new(pool);
    let service = Service::new(repository);

    // Execute CLI command
    let cli = Cli::new(root, service);
    if let Err(err) = cli.exec().await {
        let red = Styler::default().bold(true).fg(Color::Red);
        eprintln!("{}: {}", red.style("error"), err);
    }
    Ok(())
}

// Creates the directory: ~/.config/todo/
fn init() -> Result<PathBuf> {
    let root = match home::home_dir() {
        Some(mut dir) => {
            dir.push(".config");
            dir.push("todo");
            dir
        }
        None => return err!("failed to resolve home directory"),
    };

    if !root.exists() {
        std::fs::create_dir_all(&root)?;
        return err!("initialized new root at {:?}", root);
    }
    Ok(root)
}
