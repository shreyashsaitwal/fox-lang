use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        eprintln!("Usage: fox [script]")
    } else if args.len() == 2 {
        if let Some(path) = args.nth(1) {
            fox::run_file(PathBuf::from(path));
        }
    } else {
        fox::run_prompt();
    }
}
