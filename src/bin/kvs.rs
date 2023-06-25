use kvs::KvStore;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Set the value of a key.
    Set { key: String, value: String },

    /// Get the value of a key.
    Get { key: String },

    /// Remove a key.
    RM { key: String },
}

fn main() {
    let args = Args::parse();
    let mut store = KvStore::new();

    match args.command {
        Command::Set { key, value } => {
            println!("set key: {} val: {}", key, value);

            store.set(key, value);

        }
        Command::Get { key } => {
            println!("get val from key: {}", key);

            let opt = store.get(key);
            if let Some(val) = opt {
                println!("{}", val);
            }
        }
        Command::RM { key } => {
            println!("remove key from val: {}", key);

            store.remove(key);
        }
    }
}
