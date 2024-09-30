# Crate: Grammar Macros

This crate implements utility macros that implement a static parsing method on a struct. 

```rs
#[grammar{ (identifier ":")? type_name }]
struct Field {
    identifier: Option<String>,
    type_name: String,
}

#[grammar{ "struct" Identifier? "{" (Field (";" Field) *)? "}" ";" ? }]
struct Struct;
```

### Macro Documentation

**Linking a field**



- `identifier`: Expected to be declared in the struct, will parse the identifier.
- `TypeName`: Will automatically add a new field to the structure in snake case. Can only be used once per context, except if one of the types is surrounded
- `i:T` acts like `T` in grammar, but will write to identifier `i`.
- `( ... )`: Creates a new capture group.
- `... ?` converts `...`, or all children if it's a group to an option, unless it is a vector or an option already.
- `... *` converts `...`, or all children if it's a group to a vector, unless it is one already. Options are unwrapped.

### Resulting Rust Struct

The user-defined identifiers and their respective types will not be changed, however incompatibilities will be thrown, if any. If a type is defined in the grammar, it will log to the field in snake case, or create one. If the type is behind any repetition, it is wrapped into a vector, if a type is behind an optional group, it will be wrapped into an option.

The wrap casting rules are as follows: If a type `T` is alone in the grammar and required to be parsed (not optional), then it will remain as tyoe `T`. If the type is behind any repetition or declared several times, it will be wrapped into a `Vec<T>`. If a type is behind the optional group (marked by `?`), but declared once in the grammar, it will be wrapped into an `Option<T>`

To use a type within 

```rs
#[grammar{ "hello" String }]
struct Hello0; // Will generate field `string: String`

#[grammar{ "hellos" id:String ":" id2:String }]
struct Hello1; // Will generate field `id: String, id2:String`

#[grammar{ "hello" hi:String ? }]
struct Hello2; // Will generate field `hi: Option<String>`

#[grammar{ "hellos" (String ( "," String ) *) ? }]
struct Hello3; // Will generate field `string: Vec<String>`, because option casts to repetition.
```

### Open for Contribution

- Coverage of more Rust grammar syntax.
- TODO ...
