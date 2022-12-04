fn main() {
    let file_input:String = read_input_file();
    println!("{}", file_input);
}

fn read_input_file() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}