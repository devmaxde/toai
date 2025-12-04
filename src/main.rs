// src/main.rs
use arboard::Clipboard;
use clap::{ArgAction, Parser};
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "to-ai",
    version,
    about = "Dump project files to a simple AI-readable format"
)]
struct Cli {
    #[arg(long, default_value = ".", value_name = "PATH")]
    path: PathBuf,

    #[arg(long, alias = "out", value_name = "FILE", conflicts_with = "stdout")]
    output: Option<PathBuf>,

    #[arg(long, action=ArgAction::SetTrue, conflicts_with = "output")]
    stdout: bool,

    #[arg(long, action=ArgAction::Append, value_name = "PATTERN")]
    ignore: Vec<String>,

    #[arg(long = "no-ignore-default", action=ArgAction::SetTrue)]
    no_ignore_default: bool,
}

fn has_wildcards(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[') || s.contains('{')
}

fn normalize_patterns(patterns: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for p in patterns {
        let has_sep = p.contains('/') || p.contains('\\');
        if has_wildcards(p) {
            if has_sep || p.starts_with("**/") {
                out.push(p.to_string());
            } else {
                out.push(format!("**/{}", p));
            }
        } else {
            out.push(format!("**/{}/**", p));
            out.push(format!("**/{}", p));
        }
    }
    out
}

fn build_default_vec() -> Vec<&'static str> {
    vec![
        "node_modules",
        "target",
        "dist",
        "build",
        ".next",
        ".turbo",
        ".git",
        ".idea",
        ".vscode",
        ".DS_Store",
        "package-lock.json",
        "Cargo.lock",
        "LICENSE",
        "__pycache__",
        "*.pyc",
        "*.pyo",
        "*.pyd",
        "*.o",
        "*.obj",
        "*.so",
        "*.dylib",
        "*.dll",
        "*.exe",
        "*.out",
        "*.a",
        "*.lib",
        "*.log",
        "*.tmp",
        "*.swp",
        "*.png",
        "*.jpg",
        "*.jpeg",
        "*.gif",
        "*.bmp",
        "*.tiff",
        "*.ico",
        "*.svg",
        "*.webp",
        "*.heic",
        "*.heif",
        "*.vrm",
        "*.fbx",
        "*.glb",
        "*.gltf",
        "*.blend",
        "*.obj",
        "*.stl",
        "*.zip",
        "*.tar",
        "*.gz",
        "*.bz2",
        "*.xz",
        "*.7z",
        "*.rar",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        "CMakeFiles",
        "cmake-build-*",
        "buck-out",
        "bazel-*",
        "Pods",
    ]
}

fn build_globset(patterns: &[String]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for p in normalize_patterns(patterns) {
        if let Ok(g) = Glob::new(&p) {
            builder.add(g);
        }
    }
    builder.build().unwrap()
}

fn walk(
    dir: &Path,
    root: &Path,
    ignore_globs: &GlobSet,
    ignore_exact: &HashSet<PathBuf>,
    acc: &mut Vec<PathBuf>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        let rel = p.strip_prefix(root).unwrap_or(&p).to_path_buf();

        if ignore_exact.contains(&rel) {
            continue;
        }

        let ft = entry.file_type()?;
        if ignore_globs.is_match(&rel) {
            continue;
        }

        if ft.is_dir() {
            walk(&p, root, ignore_globs, ignore_exact, acc)?;
        } else if ft.is_file() {
            acc.push(rel);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut ignore_list: Vec<String> = if cli.no_ignore_default {
        Vec::new()
    } else {
        build_default_vec().into_iter().map(String::from).collect()
    };
    ignore_list.extend(cli.ignore);

    let root = fs::canonicalize(&cli.path).unwrap_or(cli.path.clone());

    let mut ignore_exact: HashSet<PathBuf> = HashSet::new();
    if let Some(out) = &cli.output {
        let out_abs = if out.is_absolute() {
            out.clone()
        } else {
            root.join(out)
        };
        if let Ok(rel) = out_abs.strip_prefix(&root) {
            ignore_exact.insert(rel.to_path_buf());
        }
    }

    let ignore_globs = build_globset(&ignore_list);

    let mut files = Vec::new();
    walk(&root, &root, &ignore_globs, &ignore_exact, &mut files)?;
    files.sort();

    let format_one = |rel: &Path| -> io::Result<String> {
        let abs = root.join(rel);
        let bytes = fs::read(&abs)?;
        let content = String::from_utf8_lossy(&bytes);
        Ok(format!("# {}\n```\n{content}```\n\n", rel.to_string_lossy()))
    };

    if let Some(out_path) = cli.output {
        // Write to file
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let file = File::create(out_path)?;
        let mut w = BufWriter::new(file);
        for rel in files {
            write!(w, "{}", format_one(&rel)?)?;
        }
        w.flush()?;
    } else if cli.stdout {
        // Write to stdout
        let stdout = io::stdout();
        let mut w = BufWriter::new(stdout.lock());
        for rel in files {
            write!(w, "{}", format_one(&rel)?)?;
        }
        w.flush()?;
    } else {
        // Default: Copy to clipboard
        let mut output = String::new();
        for rel in files {
            output.push_str(&format_one(&rel)?);
        }
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard
            .set_text(&output)
            .expect("Failed to copy to clipboard");
        eprintln!("Copied to clipboard!");
    }

    Ok(())
}
