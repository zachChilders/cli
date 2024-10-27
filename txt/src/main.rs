use clap::Subcommand;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sarcasm
    Sarcasm {
        #[arg(short, long)]
        text: String,
    },

    /// Aesthetic
    Aesthetic {
        #[arg(short, long)]
        text: String,
    },
}

fn sarcasm(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            if result.len() % 2 == 0 {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn random_kanji() -> char {
    let start = 0x4e00;
    let end = 0x9faf;
    let c = (rand::random::<u32>() % (end - start)) + start;
    std::char::from_u32(c).unwrap()
}

fn aesthetic(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            result.push(c);
            result.push(' ');
        } else {
            result.push(c);
        }
    }

    // generate 0-5 random kanji
    let mut kanji = String::new();
    for _ in 0..rand::random::<u8>() % 5
    {
        kanji.push(random_kanji());
    }

    format!("【{result}】{kanji}")
}

fn main() {
    let cli = Cli::parse();

    let text = match &cli.command {
        Commands::Sarcasm { text } => sarcasm(text),
        Commands::Aesthetic { text } => aesthetic(text)
    };

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    println!("{}", text.to_string());
    ctx.set_contents(text.to_string()).unwrap();
}
