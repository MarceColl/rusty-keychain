extern crate clap;
use clap::{Arg, App, SubCommand, ArgMatches};


fn setupCLI<'a>() -> ArgMatches<'a> {
    App::new("Rusty Keychain")
        .version("0.1")
        .author("Marcelino Coll Rovira. <coll.marce@gmail.com>")
        .about("Password manager")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("generate")
            .about("Generates a password")
            .version("0.1")
            .arg(Arg::with_name("LENGTH")
                .required(true)
                .index(2)
                .help("Length of password"))
            .arg(Arg::with_name("SITE")
                .required(true)
                .index(1)
                .help("Site the password is for")))
        .subcommand(SubCommand::with_name("sync")
            .about("Syncs the password database to git")
            .version("0.1"))
        .get_matches();
}


fn main() {
    let matches = setupCLI();
}
