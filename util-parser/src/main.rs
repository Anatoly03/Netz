use util_parser::grammar;

pub fn main() {}

// #[grammar{  }]
// // fn invoke1() {}
// struct Hello {
//     pub hi: usize,
// }

#[grammar{ (identifier ":")? type_name ";" }]
pub struct _Field {
    identifier: Option<String>,
    type_name: String,
}
