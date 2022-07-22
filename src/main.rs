use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result};
use digital_garden::write;
use directories::UserDirs;
use structopt::StructOpt;

/// A CLI for the growing and curation of a digital garden
///
/// Visit https://github.com/itsrainingmani/digital-garden for more!
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// write something in your garden
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file to your
    /// garden
    Write {
        /// Optionally set a title for your writing
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let users_dirs = UserDirs::new().ok_or_else(|| eyre!("Could not find home directory"))?;

    Ok(users_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(&opt);
    match opt.cmd {
        Command::Write { title } => write(title),
    }
}
