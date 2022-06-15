use clap::CommandFactory;
use std::path::PathBuf;

#[path="cli.rs"]
mod cli;


fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("HOME").ok_or_else(|| std::io::ErrorKind::NotFound)?);
    println!("HOME: {}", out_dir.display());
    let cmd = cli::Head::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("head.1"), buffer)?;

    Ok(())
}
