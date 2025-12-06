use clap::{Parser, Subcommand};
use svix_ksuid::*;
use ulid::Ulid;

/// UID generator from the CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Generate multiple identifiers
    #[arg(short = 'c', default_value_t = 1)]
    count: usize,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate a UUID
    Uuid,
    /// Generate a ULID
    Ulid,
    /// Generate a KSUID
    Ksuid,
}

fn gen_uuid_v4() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn gen_ulid() -> String {
    Ulid::new().to_string()
}

fn gen_ksuid() -> String {
    Ksuid::new(None, None).to_string()
}

fn uid_generator(count: usize, handler: fn() -> String) {
    for _ in 0..count {
        println!("{}", handler())
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Uuid => uid_generator(cli.count, gen_uuid_v4),
        Commands::Ulid => uid_generator(cli.count, gen_ulid),
        Commands::Ksuid => uid_generator(cli.count, gen_ksuid),
    }
}

#[cfg(test)]
mod test {}
