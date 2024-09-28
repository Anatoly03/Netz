use super::*;
use util_cases::CaseStyles;

#[test]
fn typescript_interface() {
    let mut builder = StringBuilder::from("");
    let interface_name = "Example Interface";
    let fields = vec![
        ("fieldA", "FieldType"),
        ("fieldB", "FieldType"),
        ("fieldArray", "Fields[]"),
    ];

    builder += "export interface";
    builder += format!("I{}", interface_name.to_pascal_case());
    builder += "{";
    
    builder += "}";
}
