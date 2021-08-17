use lambda_note_lib::{DocumentState, OutputFormat};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    println!("λnote - demo cli");

    let args: Vec<String> = env::args().skip(1).collect();

    let input: PathBuf = args.get(0).expect("No input file was provided").into();
    let output: PathBuf = args.get(1).expect("no output file was provided").into();

    match output.extension() {
        None => {
            println!("You need to have a file extension for the output file");
            return;
        }
        Some(extension) => match extension.to_str() {
            Some("tex") => latex(&input, &output),
            Some("html") => html(&input, &output),
            _ => {
                println!("Only .html and .tex files are valid as output");
                return;
            }
        },
    }
}

fn latex(input_file: &PathBuf, output_file: &PathBuf) {
    let content = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let mut doc = DocumentState::new(OutputFormat::Latex);
    let result = doc.translate(&content);

    fs::write(output_file, result).expect("Unable to write file");

    println!(
        "errors:\n{}\nwarnings:{}",
        doc.errors.join("\n"),
        doc.warnings.join("\n")
    );
}

fn html(input_file: &PathBuf, output_file: &PathBuf) {
    println!("Starting a live updating version");

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(input_file, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(_) => {
                println!("\n\n=== The file was rerendered ===");
                let content =
                    fs::read_to_string(input_file).expect("Something went wrong reading the file");

                let mut doc = DocumentState::new(OutputFormat::Html);
                let result = doc.translate(&content);

                fs::write(output_file, result).expect("Unable to write file");

                println!(
                    "errors:\n{}\nwarnings:{}",
                    doc.errors.join("\n"),
                    doc.warnings.join("\n")
                );
            }
            _ => (),
        }
    }
}
