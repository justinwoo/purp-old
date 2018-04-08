#[macro_use]
extern crate clap;
extern crate glob;

use glob::glob;
use clap::{App, Arg, SubCommand};
use std::process::{Command, ExitStatus};
use std::collections::LinkedList;

fn main() {
    let matches = App::new("purp")
        .version(crate_version!())
        .about("a tool for various PureScript tasks")
        .subcommand(SubCommand::with_name("build").about("Build the project"))
        .subcommand(
            SubCommand::with_name("test")
                .about("Test the project using Node.js")
                .arg(
                    Arg::with_name("main")
                        .help("Specify the main Module to be used")
                        .short("m")
                        .long("main")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("build") => run_build(),
        Some("test") => {
            let mut paths: LinkedList<String> = LinkedList::new();
            paths = push_glob(paths, "./test/**/*.purs");

            let build_status = psc_package_build(Some(paths));

            if build_status.success() {
                println!("Success. Running tests.");

                run_node_test(
                    matches
                        .subcommand_matches("test")
                        .and_then(|matches| matches.value_of("main"))
                        .unwrap_or_else(|| "Test.Main"),
                );
            } else {
                println!("Build failed.");
            }
        }
        Some(x) => println!("Unknown task: {:?}", x),
        None => run_build(),
    }

    fn run_build() {
        let build_status = psc_package_build(None);

        if build_status.success() {
            println!("Success.");
        } else {
            println!("Build failed.");
        }
    };
}

fn push_glob(mut paths: LinkedList<String>, pattern: &str) -> LinkedList<String> {
    for path in glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        paths.push_back(String::from(path.to_string_lossy()));
    }

    return paths;
}

fn run_node_test(test_main: &str) {
    let test_status = Command::new("node")
        .arg("-e")
        .arg(format!("require('./output/{}').main()", test_main))
        .spawn()
        .expect("Error in launching `node`")
        .wait()
        .expect("Error in `node`");

    if test_status.success() {
        println!("Success.");
    } else {
        println!("Test failed.");
    }
}

fn psc_package_build(paths: Option<LinkedList<String>>) -> ExitStatus {
    println!("Building...");

    return Command::new("psc-package")
        .arg("build")
        .arg("--")
        .args(paths.unwrap_or_else(|| LinkedList::new()))
        .spawn()
        .expect("Error in launching `psc-package`")
        .wait()
        .expect("Error in `psc-package`");
}
