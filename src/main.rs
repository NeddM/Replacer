use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
    // Form
    let stdin = std::io::stdin();

    // Variables files
    let vars_file: String = String::from("replace.txt");
    let vars = scan_variables(vars_file);

    // File to clone
    println!("Write the name of the file to process: ");
    let mut old_file = String::new();
    match stdin.read_line(&mut old_file) {
        Ok(read_file) => read_file,
        Err(e) => {
            println!("Error reading the prompt: {}", e);
            std::process::exit(1);
        }
    };

    // Cloned file
    println!("Write the new name of the file (Default: export.yaml): ");
    let mut new_file = String::new();

    match stdin.read_line(&mut new_file) {
        Ok(read_file) => read_file,
        Err(e) => {
            println!("Error reading the prompt: {}", e);
            std::process::exit(1);
        }
    };

    if new_file.is_empty() {
        new_file = String::from("export.yaml")
    }

    let new_workflow = create_new_file(&mut old_file, &mut new_file, vars);
    println!("{}", new_workflow);
}

fn scan_variables(file: String) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let binding = match fs::read_to_string(file) {
        Ok(bind) => bind,
        Err(e) => {
            println!("Error binding the variables: {}", e);
            std::process::exit(1);
        }
    };

    for line in binding.lines() {
        let temp_line: Vec<&str> = line.split(':').collect();
        let key = temp_line[0].trim().to_string();
        let value = temp_line[1].trim().to_string();

        vars.insert(key, value);
    }
    vars
}

fn create_new_file(old_file: &String, new_file: &String, vars: HashMap<String, String>) -> String {
    let old_data = match fs::read_to_string(&old_file.trim()) {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut new_data = old_data.clone();

    for (key, value) in &vars {
        if new_data.contains(key) {
            new_data = new_data.replace(key, value);
        }
    }

    let mut end_file = match fs::File::create(new_file.trim()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };
    match end_file.write(new_data.as_bytes()) {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let info = String::from("File exported correctly");
    info
}
