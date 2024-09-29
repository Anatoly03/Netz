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

- `identifier`: Expected to be declared in the struct, will parse the identifier.
- `TypeName`: Will automatically add a new field to the structure in snake case. Can only be used once per context, except if one of the types is surrounded
- `( ... )`: Creates a new capture group.
- `... ?` converts `...`, or all children if it's a group to an option, unless it is a vector or an option already.
- `... *` converts `...`, or all children if it's a group to a vector, unless it is one already. Options are unwrapped.

### Open for Contribution

- Coverage of more struct grammar
- TODO ...
