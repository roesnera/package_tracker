use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path
};

use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: String,

    #[arg(short, long, default_value = ".")]
    output_dir: String
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Category {
    Dev,
    Desktop,
    Entertainment,
    Core,
    Misc,
}

impl Category {
    fn all() -> [Category; 5] {
        [
            Category::Dev,
            Category::Desktop,
            Category::Entertainment,
            Category::Core,
            Category::Misc,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            Category::Dev => "dev",
            Category::Desktop => "desktop",
            Category::Entertainment => "entertainment",
            Category::Core => "core",
            Category::Misc => "misc"
        }
    }

    fn filename(&self) -> String {
        format!("{}.txt", self.as_str())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_path = Path::new(&args.input_file);
    let output_dir = Path::new(&args.output_dir);

    if !output_dir.exists() {
        std::fs::create_dir_all(output_dir)?
    }

    let mut categories = std::collections::HashMap::new();
    for category in Category::all() {
        categories.insert(category, Vec::<String>::new());
    }

    let file = File::open(input_path)
        .with_context(|| format!("Failed to open input file: {}", input_path.display()))?;
    let reader = BufReader::new(file);

    let theme = ColorfulTheme::default();
    let category_items = Category::all()
        .iter()
        .map(|c| c.as_str())
        .collect::<Vec<_>>();

    for (i, line) in reader.lines().enumerate() {
        let package = line.with_context(|| format!("Error reading line {}", i+1))?;
        if package.trim().is_empty() {
            continue;
        }

        println!("\nPackage: {}", package);
        let selection = Select::with_theme(&theme)
            .with_prompt("Select category")
            .items(&category_items)
            .default(0)
            .interact()?;

        let category = Category::all()[selection];
        categories.get_mut(&category).unwrap().push(package);
    }

    for (category, packages) in categories {
        if packages.is_empty() {
            continue;
        }

        let output_path = output_dir.join(category.filename());
        let mut file = File::create(&output_path).with_context(|| {
            format!("Failed to create output file: {}", output_path.display())
        })?;

        for package in &packages {
            writeln!(file, "{}", package)?;
        }

        println!(
            "Wrote {} packages to {}",
            packages.len(),
            output_path.display()
        );
    }

    Ok(())
}
