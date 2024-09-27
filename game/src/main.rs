use clap::Parser;

fn main() {
    let cli_args = game::cli::CliArgs::parse();
    game::run(cli_args);
}
