#[macro_use]
extern crate clap;
extern crate glob;

use glob::glob;
use clap::{App, Arg, ArgMatches, SubCommand};
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
                )
                .arg(
                    Arg::with_name("skip_build")
                        .help("Skip building the project, e.g. you have already built or use an IDE plugin")
                        .short("s")
                        .long("skip_build")
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
                    Arg::with_name("skip_build")
                        .help("Skip building the project, e.g. you have already built or use an IDE plugin")
                        .short("s")
                        .long("skip_build")
                ),
        )
        .subcommand(
            SubCommand::with_name("bundle")
                .about("Bundle the project using purs bundle. This does not bundle for the browser, you should build this further with a tool like Parcel.")
                .arg(
                    Arg::with_name("main")
                        .help("Specify the main Module to be used")
                        .short("m")
                        .long("main")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Specify the output file path (default index.js)")
                        .short("o")
                        .long("output")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("source_maps")
                        .help("Generate source maps for the bundle")
                        .long("source-maps")
                )
                .arg(
                    Arg::with_name("skip_build")
                        .help("Skip building the project, e.g. you have already built or use an IDE plugin")
                        .short("s")
                        .long("skip_build")
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("build") => run_build(),
        Some("test") => {
            let test_matches = matches.subcommand_matches("test");
            let main = test_matches
                .and_then(|matches| matches.value_of("main"))
                .unwrap_or_else(|| "Test.Main");

            let mut paths: LinkedList<String> = LinkedList::new();
            paths = push_glob(paths, "./test/**/*.purs");

            match_skip_build_and_then(test_matches, Some(paths), || {
                run_node(main);
            });
        }
        Some("run") => {
            let run_matches = matches.subcommand_matches("run");
            let main = run_matches
                .and_then(|matches| matches.value_of("main"))
                .unwrap_or_else(|| "Main");

            match_skip_build_and_then(run_matches, None, || {
                run_node(main);
            });
        }
        Some("bundle") => {
            let run_matches = matches.subcommand_matches("bundle");
            let main = run_matches
                .and_then(|matches| matches.value_of("main"))
                .unwrap_or_else(|| "Main");
            let output = run_matches
                .and_then(|matches| matches.value_of("output"))
                .unwrap_or_else(|| "index.js");
            let source_maps = run_matches
                .and_then(|matches| {
                    if matches.is_present("source_maps") {
                        Some(true)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| false);

            match_skip_build_and_then(run_matches, None, || {
                run_bundle(main, output, source_maps);
            });
        }
        Some(x) => println!("Unknown task: {:?}", x),
        None => run_build(),
    }
}

fn run_build() {
    let build_status = psc_package_build(None);

    if build_status.success() {
        println!("Success.");
    } else {
        println!("Build failed.");
    }
}

fn match_skip_build_and_then<F>(
    matches: Option<&ArgMatches>,
    paths: Option<LinkedList<String>>,
    cont: F,
) where
    F: Fn() -> (),
{
    let skip_build = matches.and_then(|matches| Some(matches.is_present("skip_build")));
    match skip_build {
        Some(true) => {
            cont();
        }
        _ => {
            let build_status = psc_package_build(paths);

            if build_status.success() {
                println!("Success.");

                cont();
            } else {
                println!("Failed.");
            }
        }
    }
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

fn run_bundle(main: &str, output: &str, source_maps: bool) {
    println!("Bundling...");

    let mut cmd = Command::new("purs");

    cmd.arg("bundle")
        .arg("./output/*/*.js")
        .arg("--module")
        .arg(main)
        .arg("--main")
        .arg(main)
        .arg("--output")
        .arg(output);

    if source_maps {
        cmd.arg("--source-maps");
    };

    let status = cmd.spawn()
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
