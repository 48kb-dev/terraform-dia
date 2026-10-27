use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use tfdia_core::SourceFile;

#[derive(Parser)]
#[command(
    name = "terraform-dia",
    version,
    about = "Generate diagrams from Terraform files"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse Terraform files and write the result.
    Generate {
        /// Directory containing .tf files.
        #[arg(long)]
        input: PathBuf,
        /// Output directory.
        #[arg(long)]
        output: PathBuf,
        /// Output file name, without extension.
        #[arg(long, default_value = "diagram")]
        name: String,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Json,
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Generate {
            input,
            output,
            name,
            format,
        } => generate(&input, &output, &name, format),
    }
}

fn generate(input: &Path, output: &Path, name: &str, format: Format) -> Result<()> {
    let files = read_tf_files(input)?;
    if files.is_empty() {
        bail!("no .tf files found in {}", input.display());
    }
    let model = tfdia_core::parse(&files)?;

    let (contents, ext) = match format {
        Format::Json => (serde_json::to_string_pretty(&model)?, "json"),
    };
    fs::create_dir_all(output).with_context(|| format!("creating {}", output.display()))?;
    let path = output.join(format!("{name}.{ext}"));
    fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}

/// Reads `*.tf` files directly in `dir` (a Terraform module is one directory, not recursive).
fn read_tf_files(dir: &Path) -> Result<Vec<SourceFile>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "tf") {
            let contents =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            files.push(SourceFile {
                path: file_name.into_owned(),
                contents,
            });
        }
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}
