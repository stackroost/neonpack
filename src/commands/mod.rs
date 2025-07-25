pub mod init;
pub mod install;
pub mod run;
pub mod r#use;
pub mod remove;
pub mod list;


use clap::Subcommand;
use remove::RemoveArgs;

#[derive(Subcommand)]
pub enum Command {
    Run(run::RunArgs),
    Init,
    Install(install::InstallArgs),
    Use { package: String },
    List(list::ListArgs),
    Remove(RemoveArgs),

}

impl Command {
    pub fn execute(self) {
        match self {
            Command::Run(args) => run::run(args),
            Command::Init => init::run(),
            Command::Install(args) => install::run(args),
            Command::Use { package } => r#use::use_package(&package),
            Command::List(args) => list::run(args),
            Command::Remove(args) => remove::run(args),
        }
    }
}
