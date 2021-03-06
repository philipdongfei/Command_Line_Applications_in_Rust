use clap::Parser;
use crate::PathBuf;


#[derive(Parser)]
pub struct Head {
    /// file to load
    #[clap(parse(from_os_str))]
    pub file: PathBuf,
    /// how many lines to print
    #[clap(short = 'n', default_value = "5")]
    pub count: usize,
}

