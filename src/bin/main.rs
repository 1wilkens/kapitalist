extern crate kapitalist;

// XXX: Remove this once it becomes obsolete
#[macro_use]
extern crate diesel_migrations;

use actix_web::{actix, server};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use diesel::{Connection, PgConnection};
use slog::{error, info, o};

use std::env;
use std::net::IpAddr;

use kapitalist::{build_app, Config};

const SUBCOMMAND_API: &str = "serve";
const SUBCOMMAND_CRON: &str = "cron";
const SUBCOMMAND_INIT: &str = "init";

embed_migrations!();

fn main() {
    // parse args
    let args = build_argparser().get_matches();

    // init logging
    let log = init_logging(&args);

    // load and check environment
    load_env();
    if let Err(var) = Config::check_env(&log) {
        error!(&log, "Failed to validate environment"; "missing" => var);
        return;
    }

    // load configuration
    let mut cfg = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!(&log, "Failed to parse configuration"; "error" => ?e);
            return;
        }
    };
    if let Some(db) = args.value_of("database") {
        cfg.db_url = db.into();
    }

    if args.subcommand_matches(SUBCOMMAND_INIT).is_some() {
        // init - init db schema
        let conn = PgConnection::establish(&cfg.db_url).expect("Could not establish connection to database");
        let _ = embedded_migrations::run_with_output(&conn, &mut std::io::stdout());
        return;
    } else if let Some(_sc) = args.subcommand_matches(SUBCOMMAND_CRON) {
        // cron - scheduled maintenance tasks
        println!("This subcommand is not implemented yet!");
        return;
    } else if let Some(sc) = args.subcommand_matches(SUBCOMMAND_API) {
        // serve - kapitalist API

        // check args and update config
        if let Some(addr) = sc.value_of("address") {
            if let Ok(ip) = addr.parse::<IpAddr>() {
                cfg.addr.set_ip(ip);
            } else {
                println!("Invalid address specified");
                return;
            }
        }
        if let Some(port) = sc.value_of("port") {
            if let Ok(p) = port.parse::<u16>() {
                cfg.addr.set_port(p);
            } else {
                println!("Invalid port specified");
                return;
            }
        }

        // init actix system
        let sys = actix::System::new("kapitalist");

        let cfg_ = cfg.clone();
        let log_ = log.clone();
        let server = server::new(move || build_app(&cfg_, &log_))
            .bind(&cfg.addr)
            .unwrap_or_else(|_| panic!("Failed to bind address from configuration: {}", &cfg.addr));

        // start server
        server.start();
        info!(&log, "Started server on: {}", &cfg.addr);
        let _ = sys.run();
    }
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
                .help("Database connection string to use (`diesel` format)")
                .takes_value(true),
        ])
        .subcommands(vec![
            SubCommand::with_name(SUBCOMMAND_API)
                .about("Serve kapitalist API")
                .args(&[
                    Arg::with_name("address")
                        .help("Which address to listen on. Overwrites value from KAPITALIST_ADDRESS")
                        .takes_value(true),
                    Arg::with_name(SUBCOMMAND_CRON)
                        .help("Which port to serve on. Overwrites value from KAPITALIST_PORT")
                        .takes_value(true),
                ]),
            SubCommand::with_name(SUBCOMMAND_INIT).about("Initialize database"),
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
