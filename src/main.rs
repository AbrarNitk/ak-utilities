fn main() {
    ak_utilities::fs::reverse(std::path::Path::new("tests/reverse1.txt")).expect("something went wrong");
    println!("Hello, world!");
}
