use crate::init;
use clap::{arg_enum, value_t, App, Arg, SubCommand};
use tokio::time::Duration;

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
        .subcommand(
            SubCommand::with_name("search-source").about("search sources").arg(Arg::with_name("source_name").required(true).index(1))
        )
        .subcommand(
            SubCommand::with_name("get-articles").about("get articles").args(&[
                    Arg::with_name("source_id").short("s").long("source").takes_value(true),
                    Arg::with_name("limit").short("l").long("limit").takes_value(true),
                    Arg::with_name("offset").short("o").long("offset").takes_value(true),
                ]
            )
        )
        .get_matches()
        .clone();

    let app = init::build_app();

    // TODO: app (tg source) must start without background
    if matches.is_present("background") {
        let app_runner = app.clone();
        tokio::spawn(async move { app_runner.run().await });
    }

    // TODO: wait until start properly
    tokio::time::delay_for(Duration::from_millis(300)).await;

    match matches.subcommand() {
        ("migrate", _) => {
            app.storage().migrate().expect("migrations failed");
        },
        ("get-articles", Some(sub_m)) => {
            let source_id: Option<i32> = sub_m.value_of("source_id").map(|v|v.parse().expect("invalid source id"));
            let limit: i64 = sub_m.value_of("limit").map(|v|v.parse().expect("invalid source id")).unwrap_or(10);
            let offset: i64 = sub_m.value_of("offset").map(|v|v.parse().expect("invalid source id")).unwrap_or(0);
            let pool = app.storage().pool();
            let found = crate::db::queries::records::get_all_records(
                &pool,
                3,  // TODO :thinking:
                source_id,
                None,
                limit,
                offset,
            )
            .await;
            println!("{:?}", found);
        }
        ("search-source", Some(sub_m)) => {
            match sub_m.value_of("source_name") {
                Some(name) => {
                    let found = app.search_source(name).await.expect("can't search source");
                    println!("{:?}", found);
                }
                None => panic!("source not specified")
            }

        }
        ("server", _) => {
            #[cfg(feature = "web")]
            {
                let pool = app.storage().pool();
                crate::server::run_server(app, pool).await.expect("can't run server");
            }

            #[cfg(not(feature = "web"))]
            {
                panic!("crate built without web feature");
            }
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
        _ => panic!("unexpected command: {:?}", matches.subcommand_name()),
    }
}
