#![allow(dead_code)]
#![allow(unused_variables)]

mod decompiler;

use crate::decompiler::Decompiler;
use clap::{App, Arg};
use console::style;
use std::path::PathBuf;

fn main() {
    let matches = App::new("apk decompiler")
        .version("0.2.0")
        .about("A super simple utility to decompile your APKs.")
        .author("Roberto Huertas <roberto.huertas@outlook.com>")
        .arg(
            Arg::with_name("file")
                .long("file")
                .short("f")
                .default_value(".")
                .takes_value(true)
                .index(1)
                .help("The path to your apk."),
        )
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let apk_path = PathBuf::from(file_path);
    let dec = Decompiler::new(apk_path);

    if let Err(e) = dec.start() {
        eprintln!("{}", style(format!("  Error: {}", e.to_string())).red());
    }
}
