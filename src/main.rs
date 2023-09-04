use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let variables = "hola.txt";
    let workflow = "hola.txt";

    read_file_line_by_line(variables);
    read_file_line_by_line(workflow);
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
