#[macro_use]
extern crate clap;

// use cinder_cli::*;

use clap::App;

fn main() {
	let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
    				.author(crate_authors!())
    				.version(crate_version!())
    				.get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    match matches.subcommand() {
        ("new", Some(new_matches)) =>{
            println!("Created {}", new_matches.value_of("JOB").unwrap());
        },
        ("queue", Some(queue_matches)) =>{
            println!("Queued {}", queue_matches.value_of("JOB").unwrap());
        },
        ("list", Some(_)) =>{
            println!("Listing all jobs");
        },
        ("", None)   => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _            => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}