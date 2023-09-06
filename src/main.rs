use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() {
    let vars_file: String = String::from("replace.txt");
    let vars = scan_variables(vars_file);

    let new_file = String::from("newworkflow.yaml");
    let old_file = String::from("workflow.yaml");
    let new_workflow = create_new_file(old_file, new_file, vars);
    println!("{}", new_workflow);
}

fn scan_variables(file: String) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let binding = fs::read_to_string(file).unwrap();

    for line in binding.lines() {
        let temp_line: Vec<&str> = line.split(':').collect();
        let key = temp_line[0].trim().to_string();
        let value = temp_line[1].trim().to_string();

        vars.insert(key, value);
    }
    vars
}

fn create_new_file(old_file: String, new_file: String, vars: HashMap<String, String>) -> String {
    let old_data = fs::read_to_string(old_file).unwrap();

    let mut new_data = old_data.clone();

    for (key, value) in &vars {
        if new_data.contains(key) {
            new_data = new_data.replace(key, value);
        }
    }

    let mut end_file = fs::File::create(new_file).unwrap();
    end_file.write(new_data.as_bytes()).unwrap();

    let info = String::from("File exported correctly");
    info
}
