mod parser;

fn main() {
    let parser = parser::parse("struct Simple {}");
    println!("{:?}", parser);
}
