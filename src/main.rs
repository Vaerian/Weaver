#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::ffi::OsString;
use std::io;

extern crate clap;
use clap::{Arg, App, AppSettings};
use clap::{crate_version, crate_description, crate_authors};

extern crate lazy_static;
use lazy_static::lazy_static;

fn main() {
    let mut app = App::new("Weaver")
        .version(crate_version!())
        .long_version({
            lazy_static! {
                static ref CACHED: String = format!("{} ({} {})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"), env!("BUILD_DATE"));
            }

            let s: &'static str = &*CACHED;
            s
        })
        .about(crate_description!())
        .author(crate_authors!(",\n"))
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(App::new("new")
            .about("Creates a new quilt project")
            .arg(Arg::new("path")
                .required(true)));

    let command_match = app.get_matches_mut();

    match command_match.subcommand() {
        Some((subcommand, args)) => match subcommand {
            "new" => println!("New"),
            _ => {}
        },
        _ => {
            app.print_help();
        }
    }
}

pub mod subcommands;