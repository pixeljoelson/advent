pub fn get_input_file(path: &str) -> String {
    // let path = std::env::args().nth(1).unwrap_or("input.txt".to_owned());
    if let Ok(input) = std::fs::read_to_string(&path) {
        input
    } else {
        eprintln!("error: file '{}' not found", &path);
        std::process::exit(1)
    }
}

#[macro_export]
macro_rules! get_input {
    () => {
        if let Some(path) = std::env::args().nth(1) {
            $crate::get_input_file(&path)
        } else {
            println!("no arguments; using built-in input");
            INPUT.to_owned()
        }
    };
}
