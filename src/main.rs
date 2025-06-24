use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn read_file(path: &Path) -> io::Result<()> {
    // File exists, ask user what to do
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    println!("\n--- File Content ---");
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    Ok(())
}
fn append_to_file(path: &Path) -> io::Result<()> {
    // --- Appending to the file ---
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)?;
    println!("Enter text to append to the file:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    let content = user_input.trim(); // remove newline
    if content.is_empty() {
        println!("No content entered. Nothing to append.");
        return Ok(());
    }
    // Append the content to the file
    writeln!(file, "{}", content.trim())?;
    println!("Text appended to file.");
    Ok(())
}
fn create_file(path: &Path) -> io::Result<()> {
    // Create a new file and write to it
    let mut file = File::create(path)?;
    println!("Enter text to write to the new file:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    writeln!(file, "{}", user_input.trim())?;
    println!("Text written to new file.");
    Ok(())
}

struct UserChoice {
    choice: char,
    filename: String,
}
fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // Remove newline and return
}
fn main() -> io::Result<()> {
    // Prompt user for filename
    println!("Enter the filename:");
    let my_path: &Path;
    let mut my_choice = UserChoice {
        choice: 'R',
        filename: String::from("filename.txt"),
    };
    let user_choice = get_user_input()?;
    // Convert to Path
    if std::any::type_name_of_val(&user_choice) == "String" && user_choice.contains('.') {
        // If the user input is a valid filename with an extension
        my_choice.filename = user_choice.clone();
        my_path = Path::new(&my_choice.filename);
        // my_path = Path::new(&user_choice);
    } else {
        // If the user input is not a valid filename, use a default filename
        my_choice.filename = String::from("default.txt");
        my_path = Path::new(&my_choice.filename);
    }


    if my_path.exists() {
        // File exists, ask user what to do
        println!("File exists. Do you want to (R)ead or (E)dit it?");
        let choice = get_user_input()?;
        let choice = choice.to_uppercase(); // Convert to uppercase
        if std::any::type_name_of_val(&choice) == "char" && choice.len() == 1 {
            my_choice.choice = choice.chars().next().unwrap_or(' ');
            match my_choice.choice {
                'R' => {
                    read_file(&my_path)?;
                }
                'E' => {
                    append_to_file(&my_path)?;
                }
                _ => {
                    println!("Invalid choice.");
                    return Ok(());
                }
            }
        }
    } else {
        // File does not exist, create and write to it
        create_file(&my_path)?;
    }

    Ok(())
}

