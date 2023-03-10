use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Simplify File Path"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Simplify {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Simplify { input }) => {
            let result = mini6::simplify_path(input);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}