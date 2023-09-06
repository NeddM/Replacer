use std::collections::HashMap;
use std::fs;
use std::io::Write;

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
        let value = temp_line[1].trim().to_string();

        vars.insert(key, value);
    }
    vars
}

fn scan_matches(old_file: String, new_file: String, vars: HashMap<String, String>) -> String {
    let old_data = fs::read_to_string(old_file).unwrap();

    let mut new_data = fs::File::create(new_file.clone()).unwrap();

    new_data.write(old_data.as_bytes()).unwrap();
    let new_data = fs::read_to_string(new_file).unwrap();

    println!("{}", new_data);

    for (key, value) in &vars {
        if new_data.contains(key) {
            let _ = new_data.replace(key, value);
        }
    }

    // for line in old_data.lines() {
    //     // println!("{}", line);

    //     // for (key, value) in &vars {
    //     if line.contains(vars.get(&"Juan")) {
    //         // if line.contains("__") {
    //         let new_line = line.replace(&vars[line].trim(), &vars[line].trim());
    //         // let new_line = line.replace(key[0], &key[&1]);
    //         match new_file.write(&new_line.as_bytes()) {
    //             Ok(_) => {}
    //             Err(err) => println!("{}", err),
    //         }
    //         match new_file.write("\n".as_bytes()) {
    //             Ok(_) => {}
    //             Err(err) => println!("{}", err),
    //         }
    //     } else {
    //     }
    //     match new_file.write(&line.as_bytes()) {
    //         Ok(_) => {}
    //         Err(err) => println!("{} \n", err),
    //     };
    //     match new_file.write("\n".as_bytes()) {
    //         Ok(_) => {}
    //         Err(err) => println!("{}", err),
    //     }
    //     // }
    // }

    let hola = String::from("hola");
    hola
}
