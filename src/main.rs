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
        .subcommand(
            SubCommand::with_name("run")
                .about("Run the project using Node.js")
                .arg(
                    Arg::with_name("main")
                        .help("Specify the main Module to be used")
                        .short("m")
                        .long("main")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("skip-build")
                        .help("Skip building the project, e.g. you have already built or use an IDE plugin")
                        .short("s")
                        .long("skip-build")
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

                run_node(
                    matches
                        .subcommand_matches("test")
                        .and_then(|matches| matches.value_of("main"))
                        .unwrap_or_else(|| "Test.Main"),
                );
            } else {
                println!("Build failed.");
            }
        }
        Some("run") => {
            let run_matches = matches.subcommand_matches("run");
            let main = run_matches
                .and_then(|matches| matches.value_of("main"))
                .unwrap_or_else(|| "Main");

            let skip_build = run_matches.and_then(|matches| Some(matches.is_present("skip-build")));
            match skip_build {
                Some(true) => {
                    run_node(main);
                }
                _ => {
                    let build_status = psc_package_build(None);

                    if build_status.success() {
                        println!("Success.");

                        run_node(main);
                    } else {
                        println!("Build failed.");
                    }
                }
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
    };}

fn push_glob(mut paths: LinkedList<String>, pattern: &str) -> LinkedList<String> {
    for path in glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        paths.push_back(String::from(path.to_string_lossy()));
    }

    return paths;
}

fn run_node(main: &str) {
    let status = Command::new("node")
        .arg("-e")
        .arg(format!("require('./output/{}').main()", main))
        .spawn()
        .expect("Error in launching `node`")
        .wait()
        .expect("Error in `node`");

    if status.success() {
        println!("Success.");
    } else {
        println!("Failed.");
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
