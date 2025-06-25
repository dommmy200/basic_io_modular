use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::env;
use console::style; // For colored output (optional, can be removed if not needed)

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
fn read_file_1(filename: &String) -> io::Result<()> {
    // File exists, ask user what to do
    let mut current_dir = env::current_dir()?;
    current_dir.push(&filename);
    
    let file = File::open(current_dir)?; // Open the file
    let reader = BufReader::new(file);

    println!("\n--- Beginning of File ---\n\n");
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }
    println!("\n\n--- End of File ---");
    Ok(())
}
fn append_to_file_1(filename: &String) -> io::Result<()> {
    // --- Appending to the file ---
    let mut current_dir = env::current_dir()?;
    current_dir.push(&filename);   // Define the file path


    let mut file = OpenOptions::new()
        .append(true)
        .open(current_dir)?;
    println!("Enter text to append to the file:");
    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input)?;
    let user_input = get_user_input()?;
    if user_input.is_empty() {
        println!("No content entered. Nothing to append.");
        return Ok(());
    }
    // Append the content to the file
    writeln!(file, "{}",user_input)?;
    println!("Text appended to file.");
    Ok(())
}
fn create_file_1(filename: &String) -> io::Result<()> {
    // Create a new file and write to it
    let mut current_dir = env::current_dir()?;
    current_dir.push(&filename);   // Define the file path
    if !current_dir.exists() {
        let mut file = OpenOptions::new()
        .write(true)
        .create_new(true) // 🔒 Fail if file exists
        .open(&mut current_dir)?;
        // Prompt user for input to write to the file
        println!("File created successfully. Enter content to write to the file:");
        let user_input = get_user_input()?;
        writeln!(file, "{}", user_input)?;
    } else {
        println!("File already exists. Do you want to overwrite it? (Y/N)");
        let user_input = get_user_input()?;
        if user_input.to_uppercase() != "Y" {
            println!("File creation cancelled.");
            return Ok(());
        }
        // If the file exists, we can still create a new one
        // but we will overwrite it
        println!("Overwriting the existing file.");
        // Open the file in write mode
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true) // 🔒 Truncate the file to zero length
            .open(&mut current_dir)?;
        writeln!(file, " ")?;
    }
    Ok(())
}

fn delete_file_1(filename: &String)-> io::Result<()> {
    // Get the current directory path
    let mut current_dir = env::current_dir()?;
    current_dir.push(&filename);
    let new_path = current_dir;
    if new_path.exists() {
        println!("Do you want to delete the file {}? (Y/N)", &filename);
        let user_input = get_user_input()?;
        if user_input.to_uppercase() != "Y" {
            println!("File deletion cancelled.");
            return Ok(());
        }
        match remove_file(new_path) {
            Ok(_) => println!("{} has been deleted successfully.", &filename),
            Err(e) => println!("Error deleting file: {}", e),
        }
    } else {
        println!("The {} file does not exist.", &filename);
    }
    Ok(())
}
fn get_user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string()) // Remove newline and return
}

fn main() -> io::Result<()> {
    // This is just a placeholder for the main function
    // The actual main function is defined above
    println!("Welcome to the Basic File Manager!");
    loop {
        clear_screen(); // Clear the screen for a fresh start
        println!("Type {} to quit at any time.", style("'Q'").bold().red());
        println!("Type {} to create a new file.", style("'R'").bold().green());
        println!("Type {} to read an existing file.", style("'C'").bold().yellow());
        println!("Type {} to edit an existing file.", style("'E'").bold().blue());
        println!("Type {} to delete an existing file.", style("'D'").bold().magenta());
        println!("Enter your choice:");
        let my_char = get_user_input()?;
        if let Some(first_char) = my_char.chars().next() {
            let choice = first_char.to_ascii_uppercase(); // normalize case
            match choice {
                'C' => {
                    println!("Creating a new file...");
                    println!("Enter the filename (with extension):");
                    // Get the filename from user input
                    let my_filename = get_user_input()?;
                    // Call create_file function here
                    create_file_1(&my_filename)?;
                },
                'R' => {
                    println!("Reading an existing file...");
                    println!("Enter the filename (with extension):");
                    // Get the filename from user input
                    let my_filename = get_user_input()?;
                    // Call read_file function here
                    read_file_1(&my_filename)?;
                },
                'E' => {
                    println!("Editing an existing file...");
                    println!("Enter the filename (with extension):");

                    let my_filename = get_user_input()?;
                    // Call append_to_file function here
                    append_to_file_1(&my_filename)?;
                },
                'D' => {
                    println!("Deleting an existing file...");
                    println!("Enter the filename (with extension):");
                    // Get the filename from user input
                    let my_filename = get_user_input()?;
                    // Call delete_file function here
                    delete_file_1(&my_filename)?;
                },
                'Q' => {
                    println!("Exiting the program.");
                    break;
                },
                _ => {
                    println!("Invalid choice. Press C, R, E, D or Q.");
                    continue;
                },
            }
        }
    }
    Ok(())
}

