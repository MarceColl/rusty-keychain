extern crate clap;
extern crate log;

use clap::{Arg, App, SubCommand, ArgMatches};


fn setup_cli<'a>() -> ArgMatches<'a> {
    App::new("Rusty Keychain")
        .version("0.1")
        .author("Marcelino Coll Rovira. <coll.marce@gmail.com>")
        .about("Password manager")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("init")
            .about("Initialize the password database")
            .version("0.1")
        	.arg(Arg::with_name("git")
        		.short("g")
                .help("Site to manage"))
        	.arg(Arg::with_name("repository")
                .required(true)
                .index(1)
                .help("Repository name")))
        .subcommand(SubCommand::with_name("manage")
            .about("Manage a site")
            .version("0.1")
            .arg(Arg::with_name("SITE")
                .required(true)
                .index(1)
                .help("Site to manage")))
        .subcommand(SubCommand::with_name("sync")
            .about("Syncs the password database to git")
            .version("0.1"))
        .get_matches()
}


fn main() {
    let matches = setup_cli();
}