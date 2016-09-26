extern crate clap;
extern crate log;
extern crate rand;

use clap::{Arg, App, ArgMatches};

mod init;
mod manage;
mod sync;

fn setup_cli<'a>() -> ArgMatches<'a> {
    App::new("Rusty Keychain")
        .version("0.1")
        .author("Marcelino Coll Rovira. <coll.marce@gmail.com>")
       	.bin_name("rk")
        .about("Password manager")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(init::define_subcommand())
        .subcommand(manage::define_subcommand())
        .subcommand(sync::define_subcommand())
        .get_matches()
}


fn main() {
    let matches = setup_cli();

    if let Some(matches) = matches.subcommand_matches("manage") {
    	manage::exec(matches);
    } else if let Some(matches) = matches.subcommand_matches("sync") {
    	sync::exec(matches);
    } else if let Some(matches) = matches.subcommand_matches("init") {
    	init::exec(matches);
    }
}