use clap::Parser;
use colored::*;
use md_to_html::MarkdownConverter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input markdown file
    #[arg(short, long)]
    input: String,

    /// Output HTML file
    #[arg(short, long)]
    output: String,

    /// CSS theme to use (default, dark, or light)
    #[arg(short, long, default_value = "default")]
    theme: String,
}

fn main() {
    let args = Args::parse();
    let converter = MarkdownConverter::new(Some(&args.theme));
    
    println!("📖 Reading markdown file: {}", args.input.bright_blue().italic());
    println!("🎨 Using theme: {}", args.theme);
    
    match converter.convert_file(&args.input, &args.output) {
        Ok(_) => {
            println!("{} Conversion completed successfully! 🎉", "SUCCESS:".green().bold());
            println!("{} Output saved to: {}", "OUTPUT:".green().bold(), args.output.bright_blue().italic());
    },
        Err(e) => {
            eprintln!("❌ Error: {}", e.to_string().red().bold());
            std::process::exit(1);
        }
    }
}
