use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    prompt: String,
}

fn main() {
    let args = Args::parse();
    println!("Your prompt: {:?}", args.prompt);
}
