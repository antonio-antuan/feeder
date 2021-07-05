use crate::db::{migrate, queries};
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
        tg,
        vk,
    }
}

impl Into<feeder::Source> for ArgSource {
    fn into(self) -> feeder::Source {
        match self {
            ArgSource::web => feeder::Source::Web,
            ArgSource::tg => feeder::Source::Telegram,
            ArgSource::vk => feeder::Source::Vk,
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
                    SubCommand::with_name("move_to_folder")
                        .args(&[
                            Arg::with_name("user_id").required(true).index(1),
                            Arg::with_name("source_id").required(true).index(2),
                            Arg::with_name("folder_id").required(true).index(3),
                        ]),
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
        )
        .subcommand(SubCommand::with_name("folders")
            .subcommands(vec![
                SubCommand::with_name("list")
                    .args(&[
                        Arg::with_name("user_id").required(true).index(1),
                    ]),
                SubCommand::with_name("add")
                    .args(&[
                        Arg::with_name("user_id").required(true).index(1),
                        Arg::with_name("folder_name").required(true).index(2),
                        Arg::with_name("parent_folder_id").required(false).index(3),
                    ]),
                SubCommand::with_name("remove")
                    .args(&[
                        Arg::with_name("user_id").required(true).index(1),
                        Arg::with_name("folder_id").required(true).index(2),
                    ]),
            ])
        );

    let app = init::build_app().await;
    let matches = cli_app.clone().get_matches();

    // TODO: app (tg source) must start without background
    if matches.is_present("background") {
        let app_runner = app.clone();
        tokio::spawn(async move { app_runner.run().await });
    }

    // TODO: wait until start properly
    tokio::time::sleep(Duration::from_millis(3000)).await;

    match matches.subcommand() {
        ("migrate", _) => {
            app.storage().migrate().expect("migrations failed");
            migrate(app.storage().pool()).expect("migrations failed");
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
                let found =
                    queries::records::get_records(&pool, user_id, source_id, None, limit, offset)
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
                let list = queries::sources::get_for_user(&app.storage().pool(), user_id).await;
                println!("{:?}", list);
            }
            ("move_to_folder", Some(move_to_folder_cmd)) => {
                let user_id = parse_arg!(move_to_folder_cmd, "user_id");
                let source_id = parse_arg!(move_to_folder_cmd, "source_id");
                let folder_id = parse_arg!(move_to_folder_cmd, "folder_id");
                queries::sources::move_to_folder(
                    &app.storage().pool(),
                    user_id,
                    source_id,
                    folder_id,
                )
                .await
                .expect("can't move folder");
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
                queries::sources::get_for_user(&db_pool, user_id)
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
            crate::grpc::server::run_server(app)
                .await
                .expect("can't run server");
        }
        ("folders", Some(folders_sub_cm)) => match folders_sub_cm.subcommand() {
            ("list", Some(folders_list_sub_cm)) => {
                let user_id = parse_arg!(folders_list_sub_cm, "user_id");
                let db_pool = app.storage().pool();
                let folders = queries::folders::get_user_folders(&db_pool, user_id)
                    .await
                    .expect("can't load user folders");
                println!("{:?}", folders);
            }
            ("add", Some(folders_add_sub_cm)) => {
                let user_id = parse_arg!(folders_add_sub_cm, "user_id");
                let folder_name = parse_arg!(folders_add_sub_cm, "folder_name");
                let parent_folder_id = folders_add_sub_cm
                    .value_of("parent_folder_id")
                    .map(|v| v.parse().unwrap());
                let db_pool = app.storage().pool();
                queries::folders::add_user_folder(&db_pool, user_id, folder_name, parent_folder_id)
                    .await
                    .expect("can't add user folder");
            }
            ("remove", Some(folders_remove_sub_cm)) => {
                let user_id = parse_arg!(folders_remove_sub_cm, "user_id");
                let folder_id = parse_arg!(folders_remove_sub_cm, "folder_id");
                let db_pool = app.storage().pool();
                queries::folders::remove_user_folder(&db_pool, user_id, folder_id)
                    .await
                    .expect("can't add user folder");
            }
            _ => panic!("unexpected command: {:?}", folders_sub_cm.subcommand_name()),
        },
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
