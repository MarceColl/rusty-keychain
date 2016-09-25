extern crate clap;
extern crate log;

use clap::{Arg, App, SubCommand, ArgMatches};

pub fn define_subcommand<'a, 'b>() -> App<'a, 'b> {
	SubCommand::with_name("manage")
        .about("Manage a site")
        .version("0.1")
        .arg(Arg::with_name("SITE")
            .required(true)
            .index(1)
            .help("Site to manage"))
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {
	println!("Manage!");
}