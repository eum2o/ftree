const EXEC_NAME: &str = "ftree";

fn main() {
    let args: Vec<String> = std::env::args().take(2).collect();
    if args.len() != 2 {
        panic!("Error: Invalid arguments.\nSyntax: {EXEC_NAME} <path>\nExample: {EXEC_NAME} .");
    }

    let path = &args[1];

    println!("Path {}", &path);
}
