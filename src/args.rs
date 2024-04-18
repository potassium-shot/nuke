use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct Args {
    /// File to nuke
    #[arg(index = 1)]
    pub file: String,

    /// Amount of bytes to nuke per tick
    #[arg(short, long, default_value_t=1 << 22)]
    pub tick_size: u64,

    /// Don't delete the file after nuking its content
    #[arg(long, default_value_t = false)]
    pub no_delete: bool,
}
