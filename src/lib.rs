pub fn get_input() -> String {
    let path = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        eprintln!("error: file '{}' not found", &path);
        std::process::exit(1)
    }
}