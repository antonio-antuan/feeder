use anyhow::{Context, Result};
use clap::{App, Arg};
use std::convert::TryFrom;
use std::{fs, path, process::exit};

fn make_app() -> App<'static, 'static> {
    App::new("gen-proto")
        .arg(
            Arg::with_name("protos")
                .short("p")
                .long("protos")
                .default_value("proto")
                .env("PROTO_PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rust_out")
                .short("r")
                .long("rust-out")
                .env("RUST_OUT_PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("js_out")
                .short("j")
                .long("js-out")
                .env("JS_OUT_PATH")
                .takes_value(true),
        )
}

fn main() -> Result<()> {
    let args = make_app().get_matches();
    if !args.is_present("js_out") && !args.is_present("rust_out") {
        eprintln!("js-out or rust-out must be presented");
        exit(1);
    }
    let js_out = args.value_of("js_out").map(|v| {
        make_dir_if_not_exists(v).expect("can't create js_out directory");
        v
    });
    let rust_out = match args.value_of("rust_out") {
        Some(d) => {
            make_dir_if_not_exists(d).expect("can't create rust_out directory");
            d
        }
        None => {
            let d = "/tmp/tonic";
            make_dir_if_not_exists(d).expect("can't create rust_out directory");
            d
        } // FIXME: delete if no needs for it
    };

    let proto_dir = path::PathBuf::try_from(args.value_of("protos").expect("protos not specified"))
        .context("can't make path")?;
    let proto_files = fs::read_dir(&proto_dir)
        .context(format!("can't read {:?}", proto_dir))?
        .into_iter()
        .map(|res| res.map(|p| p.path()))
        .collect::<Result<Vec<path::PathBuf>, std::io::Error>>()
        .context("can't get files from protos directory")?;
    let mut builder = tonic_build::configure().out_dir(rust_out);
    if let Some(j) = js_out {
        println!("{}", j);
        builder = builder
            .protoc_arg(format!("--js_out=import_style=commonjs:{}", j))
            .protoc_arg(format!(
                "--grpc-web_out=import_style=commonjs,mode=grpcwebtext:{}",
                j
            ));
    }
    builder
        .compile(&proto_files, &[proto_dir.into()])
        .context("compile failed")?;
    Ok(())
}

fn make_dir_if_not_exists(path: &str) -> Result<()> {
    if !path::Path::new(path).exists() {
        // We need to check auth errors and so on so we need to ensure that authorization not done yet.
        fs::create_dir(path)?;
    }
    Ok(())
}
