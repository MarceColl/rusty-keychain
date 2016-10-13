#[macro_use]
extern crate log;
extern crate clap;
extern crate rand;
extern crate log4rs;
extern crate yaml_rust;

use clap::{Arg, App, ArgMatches};

mod init;
mod manage;
mod sync;
mod gpg;
mod copy;
mod utils;

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
        .subcommand(copy::define_subcommand())
        .get_matches()
}


fn setup_logging() {
    log4rs::init_file("/Users/MarceColl/Projects/security/rusty-keychain/config/log4rs.yml",
                        Default::default()).unwrap();
    
    info!("------------- NEW -------------");
}


fn main() {
    let matches = setup_cli();

    setup_logging();

    if let Some(matches) = matches.subcommand_matches("copy") {
        copy::exec(matches);
    } else if let Some(matches) = matches.subcommand_matches("manage") {
    	manage::exec(matches);
    } else if let Some(matches) = matches.subcommand_matches("sync") {
    	sync::exec(matches);
    } else if let Some(matches) = matches.subcommand_matches("init") {
    	init::exec(matches);
    }
}