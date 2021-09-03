use lambda_note_lib::{DocumentState, Html, Latex, Translator};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::{env, fs, path::PathBuf, time::Duration};
use tempfile::tempdir;

/// A simple CLI program to test lambda note
/// Usage: lambda <INPUT FILE> <OUTPUT FILE>
/// If only given one arg, a file watcher and a live updating
/// server is started to live preview a html document.
fn main() {
    // collect cli args
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        1 => live_preview(args),
        2 => single_run(args),
        _ => {
            println!("Usage: lambda <INPUT FILE> <OUTPUT FILE>");
            println!("You can omit the output file to start a live html preview.");
        }
    }
}


/// Read the file once, transpile it to the correct output format and write to
/// the given output file.
fn single_run(args: Vec<String>) {
    let input_file: PathBuf = args.get(0).expect("No input file was provided").into();
    let output_file: PathBuf = args.get(1).expect("no output file was provided").into();

    match output_file.extension() {
        None => {
            println!("You need to have a file extension for the output file");
            return;
        }
        Some(extension) => match extension.to_str() {
            Some("tex") => translate(&input_file, &output_file, Latex),
            Some("html") => translate(&input_file, &output_file, Html),
            
            // The program will try to resolve non native output formats
            // by passing a tex file to pandoc.
            Some(extension_str) => {
                println!("No native support for .{} files", extension_str);
                println!("Generating a .tex file and forwarding it to pandoc");

                let mut latex_file: PathBuf =
                    output_file.clone().file_stem().expect("No filename").into();

                latex_file.set_extension("tex");

                translate(&input_file, &latex_file, Latex);
                pandoc(&latex_file, &output_file);
            }

            None => {
                println!("You need to have a file extension for the output file");
                return;
            }
        },
    }
}

/// Given a translator and input write to an output file.
fn translate<T: Translator + 'static>(input_file: &PathBuf, output_file: &PathBuf, translator: T) {
    let content = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut doc = DocumentState::new(translator);
    let result = doc.translate(&content);

    fs::write(output_file, result).expect("Unable to write file");

    println!(
        "errors:\n{}\nwarnings:{}",
        doc.errors.join("\n"),
        doc.warnings.join("\n")
    );
}


/// Invoke pandoc to translate between two formats
fn pandoc(input_file: &PathBuf, output_file: &PathBuf) {
    match Command::new("pandoc")
        .arg(input_file)
        .arg(format!("-o {}", output_file.to_string_lossy()))
        .output()
    {
        Err(error) => {
            println!("Failed to invoke pandoc. Error: {}", error);
        }
        Ok(_) => (),
    }
}

/// Start a HTML live preview.
/// Invokes the external node program "liveserver" and autoupdates  
fn live_preview(args: Vec<String>) {
    let input_file: PathBuf = args.get(0).expect("No input file was provided").into();

    println!("Starting a live updating version");

    let dir = tempdir().expect("Failed to create a temp directory");
    let output_file_path = dir.path().join("index.html");

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(&input_file, RecursiveMode::Recursive)
        .unwrap();

    translate(&input_file, &PathBuf::from(&output_file_path), Html);

    match Command::new("cmd")
        .current_dir(dir.path())
        .arg("/C")
        .arg("live-server")
        .arg("index.html")
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(error) => {
            println!("Failed to start the live-server, error: {}", error);
            return;
        }
        Ok(_) => (),
    }

    loop {
        match rx.recv() {
            Ok(_) => {
                println!("\n\n=== The file was rerendered ===");
                translate(&input_file, &PathBuf::from(&output_file_path), Html);
            }
            _ => (),
        }
    }
}
