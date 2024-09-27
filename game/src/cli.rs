use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[arg(short, long)]
    pub listen_address: Option<String>,
}
