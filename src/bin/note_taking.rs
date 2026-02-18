use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use chrono::Local;

fn main() {
    let notes_file = "notes.txt";
    
    loop {
        println!("\n=== Note-Taking Program ===");
        println!("1. Add a note");
        println!("2. Display all notes");
        println!("3. Exit");
        println!("Choose an action: ");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();
        
        match choice {
            "1" => add_note(notes_file),
            "2" => display_notes(notes_file),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn add_note(filename: &str) {
    println!("Enter your note: ");
    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read input");
    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let entry = format!("[{}] {}", timestamp, note);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .expect("Failed to open file");
    
    writeln!(file, "{}", entry).expect("Failed to write to file");
    println!("Note added successfully!");
}

fn display_notes(filename: &str) {
    let file = match std::fs::File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };
    
    let reader = BufReader::new(file);
    println!("\n=== All Notes ===");
    for (i, line) in reader.lines().enumerate() {
        if let Ok(note) = line {
            println!("{}. {}", i + 1, note);
        }
    }
    println!();
}
