extern crate clap;
extern crate log;

use clap::{Arg, App, SubCommand, ArgMatches};

pub fn define_subcommand<'a>() -> App<'a, 'a> {
	SubCommand::with_name("init")
        .about("Initialize the password database")
        .version("0.1")
    	.arg(Arg::with_name("git")
    		.short("g")
            .help("Site to manage"))
    	.arg(Arg::with_name("gpgkey")
            .required(true)
            .index(1)
            .help("gpg key id"))
}

pub fn exec<'a>(matches: &ArgMatches<'a>) {

}