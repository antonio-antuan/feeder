use crate::{init, server::run_server};
use clap::{arg_enum, value_t, App, Arg, SubCommand};
use tokio::time::Duration;
use std::process::exit;

arg_enum! {
    #[allow(non_camel_case_types)]
    pub enum ArgSource {
        web,
        tg
    }
}

impl Into<feeder::Source> for ArgSource {
    fn into(self) -> feeder::Source {
        match self {
            ArgSource::web => feeder::Source::Web,
            ArgSource::tg => feeder::Source::Telegram,
        }
    }
}
pub async fn run() {
    let matches = App::new("feeder")
        .arg(Arg::with_name("background").long("background").short("b").help("Enables background routines"))
        .subcommand(SubCommand::with_name("migrate").about("Runs databases migration"))
        .subcommand(
            SubCommand::with_name("sync").about("Synchronizes sources").args(&[
                Arg::with_name("source")
                    .help("specify particular source for synchronization, synchronizes all sources if not specified ")
                    .short("s")
                    .long("source")
                    .takes_value(true)
                    .possible_values(&ArgSource::variants()),
                Arg::with_name("secs_depth")
                    .help("seconds ago for source")
                    .short("d")
                    .long("secs")
                    .takes_value(true),
            ]),
        )
        .subcommand(SubCommand::with_name("server").about("runs web server"))
        .get_matches()
        .clone();

    let app = init::build_app();

    // TODO: app (tg source) must start without background
    if matches.is_present("background") {
        let app_runner = app.clone();
        tokio::spawn(async move { app_runner.run().await });
    }

    // TODO: wait until start properly
    tokio::time::sleep(Duration::from_millis(300)).await;

    match matches.subcommand() {
        ("migrate", _) => {
            app.storage().migrate().expect("migrations failed");
        }
        ("server", _) => {
            let pool = app.storage().pool();
            run_server(app, pool).await.expect("can't run server");
        }
        ("sync", Some(sub_m)) => {
            let secs = value_t!(sub_m, "secs_depth", i32).expect("can't parse secs argument");
            let source = sub_m
                .value_of("source")
                .map(|v| v.parse::<ArgSource>().expect("get invalid source"));

            app.synchronize(secs, source.map(|s| s.into()))
                .await
                .expect("can't synchronize")
        }
        (_, None) => {
            eprintln!("command not specified");
            exit(1)
        },
        _ => panic!("unexpected command: {:?}", matches.subcommand_name()),
    }
}
