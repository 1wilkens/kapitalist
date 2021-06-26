// XXX: Remove this once it becomes obsolete
#[macro_use]
extern crate diesel_migrations;

use std::env;
use std::net::IpAddr;
use std::sync::Arc;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{Connection, PgConnection};
use tracing::Level;

use kapitalist::{db, AppState, Config};

const SUBCOMMAND_API: &str = "serve";
const SUBCOMMAND_CRON: &str = "cron";
const SUBCOMMAND_INIT: &str = "init";

embed_migrations!();

#[tokio::main]
async fn main() -> Result<(), String> {
    // parse args
    let args = build_argparser().get_matches();

    // init logging
    init_logging(&args)?;

    // load and check environment
    import_env();
    Config::check_env()?;

    // load and check configuration
    let mut cfg =
        Config::from_env().map_err(|e| format!("Failed to parse configuration: {:?}", e))?;
    if let Some(db) = args.value_of("database") {
        cfg.db_url = db.into();
    }

    // execute subcommand
    if args.subcommand_matches(SUBCOMMAND_INIT).is_some() {
        // init - init db schema
        let conn = PgConnection::establish(&cfg.db_url)
            .map_err(|err| format!("Failed to establish connection to database: {}", err))?;
        return embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
            .map_err(|err| format!("Error during database initialization: {}", err));
    } else if let Some(_sc) = args.subcommand_matches(SUBCOMMAND_CRON) {
        // cron - scheduled maintenance tasks
        return kapitalist::cron::execute(&cfg);
    } else if let Some(sc) = args.subcommand_matches(SUBCOMMAND_API) {
        // serve - kapitalist API

        // check args and update config
        if let Some(addr) = sc.value_of("address") {
            // check if we got a valid IP
            if let Ok(a) = addr.parse::<IpAddr>() {
                cfg.address = a;
            } else {
                return Err(format!("Invalid address specified: {}", addr));
            }
        }
        if let Some(port) = sc.value_of("port") {
            // check if we got a valid port
            if let Ok(p) = port.parse::<u16>() {
                cfg.port = p;
            } else {
                return Err(format!("Invalid port specified: {}", port));
            }
        }

        // setup app state and database connection
        let st = Arc::new(AppState::new(cfg.clone()).build());
        let db = db::init_pool(&cfg.db_url);
        let site = kapitalist::build_site(st, db);

        warp::serve(site).run((cfg.address, cfg.port)).await;
    }
    Ok(())
}

fn build_argparser<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .args(&[
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Only print errors")
                .takes_value(false),
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Print more verbose output")
                .takes_value(false),
            Arg::with_name("debug")
                .long("debug")
                .help("Print debug output (implies --verbose)")
                .takes_value(false),
            Arg::with_name("database")
                .short("d")
                .long("database")
                .help("Database connection string to use (diesel format)")
                .takes_value(true),
        ])
        .subcommands(vec![
            SubCommand::with_name(SUBCOMMAND_API)
                .about("Serve kapitalist API")
                .args(&[
                    Arg::with_name("host")
                        .long("host")
                        .help("Which host to listen on. Overwrites value from KAPITALIST_HOST")
                        .takes_value(true),
                    Arg::with_name("port")
                        .long("port")
                        .help("Which port to serve on. Overwrites value from KAPITALIST_PORT")
                        .takes_value(true),
                ]),
            SubCommand::with_name(SUBCOMMAND_INIT).about("Initialize database and exit"),
            SubCommand::with_name(SUBCOMMAND_CRON)
                .about("Perform scheduled maintenance tasks and exit"),
        ])
}

fn init_logging(args: &ArgMatches) -> Result<(), String> {
    let level = if args.is_present("debug") {
        Level::DEBUG
    } else if args.is_present("verbose") {
        Level::INFO
    } else if args.is_present("quiet") {
        Level::ERROR
    } else {
        Level::WARN
    };

    let subscriber = tracing_subscriber::fmt().with_max_level(level).finish();
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| format!("Failed to set global subscriber: {}", e))
}

fn import_env() {
    if let Ok(variables) = dotenv::dotenv_iter() {
        for (key, val) in variables.flatten() {
            if env::var(&key) == Err(env::VarError::NotPresent) {
                env::set_var(&key, &val);
            }
        }
    }
}
