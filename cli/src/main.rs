extern crate clap;
extern crate log;
extern crate pretty_env_logger;

use clap::{AppSettings, Clap};

mod dependency;

#[derive(Clap)]
#[clap(name = "styx", version = "0.1.0", author = "skyezerfox (Skye) <actuallyori@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::DisableHelpSubcommand)]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Test(Test),
    Init(Init),
    Dependency(Dependency)
}


/// Initialize a new Styx project
#[derive(Clap)]
struct Init {

}

/// Manage the dependencies of the current project.
#[derive(Clap)]
#[clap(aliases = &["dep"])]
struct Dependency {
    
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {}

fn main() {
    let opts = Opts::parse();

    pretty_env_logger::formatted_builder().init();
}
