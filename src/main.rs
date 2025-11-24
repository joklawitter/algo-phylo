use nexus_parser::parse_nexus_file;

mod model;
mod parser;

fn main() {
    let file = "path/to/your/trees/file/foo.trees";
    // let start = Instant::now();
    let (trees, _) = parse_nexus_file(file).unwrap();
    // let duration = start.elapsed();
    // println!("Parsing took: {:?}", duration);
    print!("num trees: {}", trees.len());
}
