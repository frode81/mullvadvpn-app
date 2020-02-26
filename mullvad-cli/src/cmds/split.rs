use crate::{Command, Result};

pub struct Split;

impl Command for Split {
    fn name(&self) -> &'static str {
        "split-tunnel"
    }

    fn clap_subcommand(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name())
            .about("Manage split tunneling")
            .setting(clap::AppSettings::SubcommandRequiredElseHelp)
            .subcommand(create_pid_subcommand())
    }

    fn run(&self, matches: &clap::ArgMatches<'_>) -> Result<()> {
        match matches.subcommand() {
            ("pid", Some(pid_matches)) => Self::handle_pid_cmd(pid_matches),
            _ => unreachable!("unhandled comand"),
        }
    }
}

fn create_pid_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("pid")
        .about("Manage processes to exclude from the tunnel")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            clap::SubCommand::with_name("add").arg(clap::Arg::with_name("pid").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("delete").arg(clap::Arg::with_name("pid").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("list")
        )
}

impl Split {
    fn handle_pid_cmd(matches: &clap::ArgMatches<'_>) -> Result<()> {
        match matches.subcommand() {
            ("add", Some(matches)) => {
                // TODO
                Ok(())
            }
            ("delete", Some(matches)) => {
                // TODO
                Ok(())
            }
            ("list", Some(matches)) => {
                // TODO
                Ok(())
            }
            _ => unreachable!("unhandled command"),
        }
    }
}
