use anyhow::Result;
use clap::Parser;
use hashy::cli::Cli;
use hashy::command::handle_hash;
use hashy::verbosity::Verbosity;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging/verbosity
    let verbosity = if cli.quiet {
        Verbosity::Quiet
    } else if cli.verbose {
        Verbosity::Verbose
    } else {
        Verbosity::Normal
    };

    if let Some((
        algo,
        allow_insecure,
        text,
        file,
        files,
        continue_on_error,
        format,
        uppercase,
        json,
    )) = cli.command.get_hash_params()
    {
        handle_hash(
            algo,
            allow_insecure,
            text,
            file,
            files,
            continue_on_error,
            format,
            uppercase,
            json,
            verbosity,
        )?;
    }

    Ok(())
}
