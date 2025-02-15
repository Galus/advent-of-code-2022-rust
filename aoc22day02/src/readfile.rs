use std::fs;

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath).unwrap();
    Ok(data)
}

pub fn readfile() -> String {
    let input = match read_file_string("input.txt") {
        Ok(data) => data,
        Err(err) => panic!(
            "Make sure input.txt is in same path cargo run is called. Err: {:?}",
            err
        ),
    };
    // println!("{:?}", input);
    input
}
