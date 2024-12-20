use anyhow::{bail, Result};
use cargo_metadata::{camino::Utf8PathBuf, Message};
use clap::{Args, Parser};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug, Args)]
struct BuildArgs {
    /// Optional binaries to act on (all binaries used if omitted)
    bin: Option<String>,
}

#[derive(Debug, Parser)]
enum Cli {
    /// Build binaries
    Build(BuildArgs),
}

fn main() -> Result<()> {
    let workspace = std::env::current_dir()?;

    match Cli::parse() {
        Cli::Build(args) => build_binary(&workspace, args),
    }
}

/// Build one or more binaries
fn build_binary(workspace: &Path, args: BuildArgs) -> Result<()> {
    let package_path_buf = workspace.join("servox");
    let package_path = package_path_buf.as_path();
    let bin_artifacts = cargo_build(package_path, &args.bin)?;
    for artifact in bin_artifacts.iter() {
        let bin_path = generate_binary(package_path, artifact)?;
        generate_hex(&bin_path)?;
    }
    Ok(())
}

/// Create a hex memory file from a binary file
fn generate_hex(bin_path: &Utf8PathBuf) -> Result<()> {
    let hex_path_buf = bin_path.with_extension("hex");
    let hex_path = hex_path_buf.as_path();

    let mut reader: BufReader<File> = BufReader::new(File::open(bin_path)?);
    let mut writer: BufWriter<File> = BufWriter::new(File::create(hex_path)?);

    loop {
        let mut word_bytes = [0u8; 4];
        match reader.read(&mut word_bytes) {
            Ok(size) => {
                let word = u32::from_le_bytes(word_bytes);
                let _ = writeln!(writer, "{:08X}", word);
                if size < word_bytes.len() {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(())
}

/// Create a binary from a ELF artifact
fn generate_binary(
    package_path: &Path,
    artifact: &cargo_metadata::Artifact,
) -> Result<Utf8PathBuf> {
    if let Some(path) = &artifact.executable {
        let bin_path = path.with_extension("bin");
        let mut tool = Command::new("rust-objcopy");
        tool.arg("-O");
        tool.arg("binary");
        tool.arg(path.to_string());
        tool.arg(bin_path.to_string());
        let status = tool
            .current_dir(package_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .status()?;
        if status.success() {
            Ok(bin_path)
        } else {
            bail!("Failed to execute cargo subcommand")
        }
    } else {
        bail!("No executable built")
    }
}

/// Build release bibnary with cargo build
fn cargo_build(
    package_path: &Path,
    name: &Option<String>,
) -> Result<Vec<cargo_metadata::Artifact>> {
    let mut args = vec![
        "build".to_string(),
        "--release".to_string(),
        "--message-format=json".to_string(),
    ];

    if let Some(name) = name {
        args.push(format!("--bin={}", name));
    } else {
        args.push("--bins".to_string());
    }

    let mut child = Command::new("cargo")
        .args(args)
        .current_dir(package_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()?;

    let stdout = BufReader::new(child.stdout.take().expect("Pipe to cargo process failed"));

    let messages = Message::parse_stream(stdout).collect::<Vec<_>>();

    let status = child.wait()?;
    if !status.success() {
        bail!("Failed to execute cargo subcommand")
    }

    let mut bin_artifacts = Vec::new();

    for message in messages {
        match message? {
            Message::CompilerArtifact(artifact) => {
                if artifact
                    .target
                    .kind
                    .contains(&cargo_metadata::TargetKind::Bin)
                {
                    bin_artifacts.push(artifact);
                }
            }
            Message::CompilerMessage(msg) => {
                if let Some(rendered) = msg.message.rendered {
                    print!("{rendered}");
                }
            }
            _ => (),
        }
    }
    Ok(bin_artifacts)
}
