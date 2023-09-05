use std::collections::HashMap;
use std::fs;

fn main() {
    let vars_file: String = String::from("variables.txt");
    let vars = scan_variables(vars_file);

    let new_file = String::from("newworkflow.yaml");
    let old_workflow = String::from("workflow.yaml");
    let new_workflow = scan_matches(old_workflow, new_file, vars);
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

fn scan_matches(old_file: String, new_file: String, vars: HashMap<String, String>) -> String {
    fs::File::create(new_file.clone()).unwrap();
    let old_data = fs::read_to_string(old_file).unwrap();
    // let mut new_data = fs::File::open(new_file).unwrap();

    // for line in old_data.lines() {
    //     println!("{}", line);
    //     let _ = new_data.write_all(&line.as_bytes());
    // }

    let mut new_data = old_data.clone();

    let new_data = new_data.replace(vars[0], vars[1]);

    let hola = String::from("hola");
    hola
}
