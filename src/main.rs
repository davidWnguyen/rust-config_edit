use clap::Parser;
use evalexpr::*;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
    pattern: String,
    eval: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)?;
    let mut context = HashMapContext::new();

    let mut file = OpenOptions::new()
        .write(true)
        .open(&args.path)
        .unwrap();

    for line in content.lines() {
        if line.contains(('"'.to_string() + &args.pattern).as_str()) {
            let digits: String = line.chars().filter(|char| char.is_numeric() || *char == '.').collect();
            let number: f64 = digits.parse::<f64>().unwrap();
            context.set_value("x".into(), number.into()).unwrap();

            let equation_output:String = eval_number_with_context_mut(&args.eval.as_str(), &mut context).unwrap().to_string();
            
            match file.write((line.replace(digits.as_str(), equation_output.as_str()) + "\n").as_bytes()){
                Err(_) => panic!("Failed to write to file."),
                _ => ()
            };
            println!("{} -> {}", line, equation_output);
        }
        else{
            match file.write((line.to_string() + "\n").as_bytes()){
                Err(_) => panic!("Failed to write to file."),
                _ => ()
            };
        }
    }
    Ok(())
}
