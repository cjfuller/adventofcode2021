use std::path::Path;

pub fn load_input(day: u16) -> String {
    let filename = Path::new("./inputs").join(&format!("day_{}.txt", day));
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string()
}

pub fn input_lines(day: u16) -> Vec<String> {
    load_input(day)
        .lines()
        .map(|it| it.trim().to_string())
        .collect()
}
