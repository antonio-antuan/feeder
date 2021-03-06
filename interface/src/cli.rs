use crate::db::queries;
use crate::init;
use clap::{arg_enum, value_t, App, Arg, SubCommand};
use std::process::exit;
use tokio::time::Duration;

macro_rules! parse_arg {
    ($cmd:expr, $arg:expr) => {
        $cmd.value_of($arg)
            .expect(format!("{} not specified", $arg).as_str())
            .parse()
            .expect(format!("invalid {} specified", $arg).as_str())
    };
}

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
    let cli_app = App::new("feeder")
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
            SubCommand::with_name("sources")
                .subcommands(vec![
                    SubCommand::with_name("list").about("list sources")
                        .arg(
                            Arg::with_name("user_id").required(true).index(1),
                        ),
                    SubCommand::with_name("search").about("search sources")
                        .arg(
                            Arg::with_name("source_name").required(true).index(1)
                        ),

                    SubCommand::with_name("subscribe").about("list articles")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("source_id").required(true).index(2),
                        ])
                ])
        )
        .subcommand(
            SubCommand::with_name("articles")
                .subcommands(vec![

                    SubCommand::with_name("list")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("source_id").short("s").long("source").takes_value(true),
                            Arg::with_name("limit").short("l").long("limit").takes_value(true),
                            Arg::with_name("offset").short("o").long("offset").takes_value(true),
                        ]),

                    SubCommand::with_name("star")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("record_id").required(true).index(2),
                            Arg::with_name("unstar").short("u").long("unstar")
                        ]),

                    SubCommand::with_name("add_tag")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("record_id").required(true).index(2),
                            Arg::with_name("tag").required(true).index(3),
                        ]),

                    SubCommand::with_name("remove_tag")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("record_id").required(true).index(2),
                            Arg::with_name("tag").required(true).index(3),
                        ]),
                ])
        );

    let app = init::build_app();
    let matches = cli_app.clone().get_matches();
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
        }
        ("articles", Some(articles_command)) => match articles_command.subcommand() {
            ("list", Some(list_command)) => {
                let user_id = parse_arg!(list_command, "user_id");
                let source_id = list_command
                    .value_of("source_id")
                    .map(|v| v.parse().expect("invalid source id"));
                let limit = list_command
                    .value_of("limit")
                    .map(|v| v.parse().expect("invalid source id"))
                    .unwrap_or(10);
                let offset = list_command
                    .value_of("offset")
                    .map(|v| v.parse().expect("invalid source id"))
                    .unwrap_or(0);
                let pool = app.storage().pool();
                let found = queries::records::get_all_records(
                    &pool, user_id, source_id, None, limit, offset,
                )
                .await
                .expect("can't get records");
                println!("{:?}", found);
            }

            ("star", Some(star_cmd)) => {
                let user_id = parse_arg!(star_cmd, "user_id");
                let record_id = parse_arg!(star_cmd, "record_id");
                let unstar = star_cmd.is_present("unstar");
                queries::records::mark_record(&app.storage().pool(), user_id, record_id, !unstar)
                    .await
                    .expect("can't perform record starring");
            }

            ("add_tag", Some(add_tag)) => {
                let user_id = parse_arg!(add_tag, "user_id");
                let record_id = parse_arg!(add_tag, "record_id");
                let tag = parse_arg!(add_tag, "tag");
                queries::records::add_tag(&app.storage().pool(), user_id, record_id, tag)
                    .await
                    .expect("can't perform tag addition");
            }

            ("remove_tag", Some(remove_tag)) => {
                let user_id = parse_arg!(remove_tag, "user_id");
                let record_id = parse_arg!(remove_tag, "record_id");
                let tag = parse_arg!(remove_tag, "tag");
                queries::records::remove_tag(&app.storage().pool(), user_id, record_id, tag)
                    .await
                    .expect("can't perform tag removing");
            }
            _ => panic!(
                "unexpected command: {:?}",
                articles_command.subcommand_name()
            ),
        },
        ("sources", Some(sources_command)) => match sources_command.subcommand() {
            ("", _) => {
                eprintln!("subcommand not specified");
                exit(1)
            }
            ("list", Some(list_sub_cmd)) => {
                let user_id = parse_arg!(list_sub_cmd, "user_id");
                let list = queries::sources::get_list(&app.storage().pool(), user_id).await;
                println!("{:?}", list);
            }
            ("search", Some(search_sub_cm)) => {
                let search = search_sub_cm
                    .value_of("source_name")
                    .expect("source not specified");
                let found = app
                    .search_source(search)
                    .await
                    .expect("can't search source");
                println!("{:?}", found);
            }
            ("subscribe", Some(subscribe_sub_cm)) => {
                let source_id = parse_arg!(subscribe_sub_cm, "source_id");
                let user_id = parse_arg!(subscribe_sub_cm, "user_id");
                let db_pool = app.storage().pool();
                queries::sources::subscribe(&db_pool, source_id, user_id)
                    .await
                    .expect("subscription failed");
                queries::sources::get_list(&db_pool, user_id)
                    .await
                    .map(|v| v.into_iter().find(|s| s.id == source_id))
                    .expect("subscription failed")
                    .map(|_| println!("subscription created"));
            }
            _ => panic!(
                "unexpected command: {:?}",
                sources_command.subcommand_name()
            ),
        },
        ("server", _) => {
            #[cfg(feature = "web")]
            {
                let pool = app.storage().pool();
                crate::server::run_server(app, pool)
                    .await
                    .expect("can't run server");
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
