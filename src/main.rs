use std::path::Path;
use std::env;

fn main() -> Result<(), String> {
    let path = env::current_dir().unwrap();
    let file = env::args().skip(1).next().unwrap();
    let file = Path::new(&file);

    for ancestor in path.ancestors() {
        let target = ancestor.join(file);
        if target.exists() {
            println!("{}", target.display());
            return Ok(());
        }
    }
    Err(String::from("File not found in ancestor directories."))
}
