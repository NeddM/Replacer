use std::collections::HashMap;
use std::{fs, string};

fn main() {
    let vars_file: String = String::from("variables.txt");
    let vars = scan_variables(vars_file);

    let old_workflow = String::from("workflow.yaml");
    let new_workflow = scan_matches(old_workflow, vars);
    println!("{}", new_workflow);
}

fn scan_variables(file: String) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let binding = fs::read_to_string(file).unwrap();

    for line in binding.lines() {
        let temp_line: Vec<&str> = line.split(':').collect();
        let key = temp_line[0].trim().to_string();
        let value = temp_line[0].trim().to_string();
        vars.insert(key, value);
    }
    vars
}

fn scan_matches(old_file: String, vars: HashMap<String, String>) -> String {
    fs::File::create("newworkflow.yaml").unwrap();
    let mut old_file = fs::read_to_string("workflow.yaml").unwrap();
    let mut new_file = fs::read_to_string("newworkflow.yaml").unwrap();

    for line in old_file.lines() {
        println!("{}", line);
        fs::write(&mut new_file, line);
    }
    println!("{}", new_file);
    let hola = String::from("hola");
    hola
}
