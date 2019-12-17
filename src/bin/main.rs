#![feature(proc_macro_hygiene, decl_macro)]
extern crate kapitalist;

// XXX: Remove this once it becomes obsolete
#[macro_use]
extern crate diesel_migrations;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{Connection, PgConnection};
use slog::o;

use std::env;
use std::net::IpAddr;

use kapitalist::{Config};

const SUBCOMMAND_API: &str = "serve";
const SUBCOMMAND_CRON: &str = "cron";
const SUBCOMMAND_INIT: &str = "init";

embed_migrations!();

fn main() -> Result<(), String> {
    // parse args
    let args = build_argparser().get_matches();

    // init logging
    let log = init_logging(&args);

    // load and check environment
    load_env();
    if let Err(var) = Config::check_env(&log) {
        return Err(format!("Failed to validate environment: Missing variable {}", var));
    }

    // load and check configuration
    let mut cfg = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            return Err(format!("Failed to parse configuration: {:?}", e));
        }
    };
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
        return kapitalist::cron::execute(&cfg, &log);
    } else if let Some(sc) = args.subcommand_matches(SUBCOMMAND_API) {
        // serve - kapitalist API

        // check args and update config
        if let Some(addr) = sc.value_of("address") {
            // check if we got a valid IP
            if addr.parse::<IpAddr>().is_ok() {
                cfg.address = addr.to_string();
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

        let rocket = kapitalist::build_rocket(&cfg, &log);

        // start server
        rocket.launch();
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
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Print more verbose output")
                .takes_value(false),
            Arg::with_name("debug")
                .long("debug")
                .help("Print debug output (implies --verbose)"),
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
                    Arg::with_name("address")
                        .long("address")
                        .help("Which address to listen on. Overwrites value from KAPITALIST_ADDRESS")
                        .takes_value(true),
                    Arg::with_name("port")
                        .long("port")
                        .help("Which port to serve on. Overwrites value from KAPITALIST_PORT")
                        .takes_value(true),
                ]),
            SubCommand::with_name(SUBCOMMAND_INIT).about("Initialize database and exit"),
            SubCommand::with_name(SUBCOMMAND_CRON).about("Perform scheduled maintenance tasks and exit"),
        ])
}

fn init_logging(args: &ArgMatches) -> slog::Logger {
    use slog::Drain;

    let log_level = if args.is_present("debug") {
        slog::Level::Debug
    } else if args.is_present("verbose") {
        slog::Level::Info
    } else {
        slog::Level::Error
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, o!())
}

fn load_env() {
    if let Ok(variables) = dotenv::dotenv_iter() {
        for item in variables {
            if let Ok((key, val)) = item {
                if let Err(env::VarError::NotPresent) = env::var(&key) {
                    env::set_var(&key, &val);
                }
            }
        }
    }
}
