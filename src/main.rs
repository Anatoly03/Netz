mod parser;

fn main() {
    let parser = parser::reader::parse("struct Simple {}");
    println!("{:?}", parser);
}
