use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    loop {
        println!("=====================================");
        println!("Welcome to the file reader program!");
        println!("Enter the file path:");
        let mut file_path = String::new();
        println!("=====================================");
        io::stdin().read_line(&mut file_path).expect("Failed to read line");
        let file_path = file_path.trim().replace("\n\r", "");
        let file_path = file_path.trim_matches('"');
        println!("=====================================");
        println!("Is the provided path correct? (y/n): {}", file_path);
        println!("=====================================");
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).expect("Failed to read line");
        let confirm = confirm.trim().to_lowercase().replace("\n\r", "");
        println!("=====================================");
        if confirm == "y" {
            let file = File::open(&file_path);
            match file {
                Ok(_) => {
                    println!("File found at path: {}", &file_path);
                    println!("=====================================");
                    read_file(file_path).expect("Failed to read file");
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e);
                    println!("Could not find file at provided path.");
                    println!("=====================================");
                    println!("Do you want to try again? (y/n): ");
                    println!("=====================================");
                    let mut try_again = String::new();
                    io::stdin().read_line(&mut try_again).expect("Failed to read line");
                    let try_again = try_again.trim().to_lowercase().replace("\n\r", "");
                    if try_again == "n" {
                        break;
                    }
                    else if try_again == "y" {
                        continue;
                    }
                    else {
                        println!("Invalid input. Continuing...");
                        continue;
                    }
                }
            }
        }
        else if confirm == "n" {
            continue;
        }
        else {
            println!("Invalid input. Continuing...");
            continue;
        }
    }
}

fn read_file(file_path: &str)  -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents:");
    println!("{}", contents);
    println!("=====================================");
    println!("Press enter to continue...");
    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt).expect("Failed to read line");
    println!("=====================================");
    println!("Would you like to read another file? (y/n): ");
    println!("=====================================");
    let mut another_file = String::new();
    io::stdin().read_line(&mut another_file).expect("Failed to read line");
    if another_file.trim().to_lowercase() == "y" {
        main();
    }
    else {
        println!("Exiting program...");
    }
    Ok(())
}
