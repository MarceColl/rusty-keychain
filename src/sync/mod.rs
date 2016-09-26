extern crate clap;
extern crate log;

use clap::{App, SubCommand, ArgMatches};

pub fn define_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("sync")
        .about("Syncs the password database to git")
        .version("0.1")
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	println!("Sync!");
}