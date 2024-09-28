# Network File Format

The `network-parser` module focuses on parsing the Network File, as specified in this Markdown document. This file type is used for defining structured data with universal accross programming languages encoding, which makes it possible to exchange data between languages, as well as client to server communication.

## Custom Types

All custom types in NetFile have to follow a strict [identifier case](https://stackoverflow.com/a/54330161/16002144), such a type definitions need `CapitalCamelCase`, while all field names require you to use `snake_case`. This is a semantic restriction built into the parser to tokenize types and field names differently. Note, that primitive types will still follow `flatcase` and are a different set of tokens in the parser, so these should not be confused with field names or custom types.

### Structures

- [x] Syntax Highlight Defined
- [ ] Parser Implemented
- [ ] Parser Tests Implemented
- [ ] Generator Implemented
- [ ] Generator Tests Implemented

A structure is a dictionairy template with a limited amount of fields representing the allowed dictionairy keys and their types representing fixed dictionairy values. A shortcut to omit the field name is possible by simply writing the referenced type name. It will be automatically generated with the proper field name in snake case.

Optional fields can be annotated with the option keyword, followed by the type in round brackets. Think of it like a generic `Option<T>` like in Rust, or the optional marker `?` in TypeScript. This value will be clearly marked as potentially undeclared depending on different languages.

TODO encoding convention

```net
struct Author {
    id: u(128);
    name: string;
}

struct Message {
    Author;
    content: string;
    edited: option(string);
}
```

### Types

- [x] Syntax Highlight Defined
- [ ] Parser Implemented
- [ ] Parser Tests Implemented
- [ ] Generator Implemented
- [ ] Generator Tests Implemented

Custom types can be created from primitives, these will be treated differently during code generation, but will appear as usual custom types for the parser and can be used as such.

```net
@Sanitze("HTML")
type Content = string;

struct Message {
    Content;
}
```

```
@ConvertTo("ASCII")
type string = u8[];
```

### Enumerables

- [ ] Syntax Highlight Defined
- [ ] Parser Implemented
- [ ] Parser Tests Implemented
- [ ] Generator Implemented
- [ ] Generator Tests Implemented

TODO rights and permission enums, enums, flags, selects, etc.

Note, enum members are declared locally and don't register their presence globally.

```net
enum MessageType {
    Post = 0;
    Topic = 1;
    Thread = 2;
}
```

### Protocol

- [ ] Syntax Highlight Defined
- [ ] Parser Implemented
- [ ] Parser Tests Implemented
- [ ] Generator Implemented
- [ ] Generator Tests Implemented

A protocol is speaking simply, the entry type to encode and decode from a running websocket. Both client and server have to agree to a common protocol, which all encoding and decoding starts from, although custom implementations can using different codings directly from a structure, this is the recommended way to go with. A protocol, just like an enum starts a message by sending a discriminator. The type of the discriminator can be set just like with enums.

All types have to be declared outside of the protocol globally. If a type is common across senders, you can simply write the type name followed by an enum-like discriminator assignment, see `Ping = 0;`. The types can differ on the who plays the role of the receiver and sender. While there can be several agents, NetFile will only classify two: The Client, the agent who sends "requests" and initiates the connection and the Server, the agent who responds. As such the grammar is a right-directed arrow from the client to the server. `ClientType -> ServerType = Discriminator;`

If there is a discriminator, in which only one agent will send, then the primitive unit type `()` can be replaced instead of an agent. In the example below, it is assumed that client and server communicate with pings over the socket, the client can send HTTP requests and get responses over an open socket, and inbetween the server can send new messages.

Don't forget that a protocol is a custom type in disguise, and fields can have their names specified. You could for example define the fields `different: Custom = 0;` as well as `from: From -> to: To = 1;`, however struct nesting is not allowed in a protocol definition. If the field is omitted, typical rules apply.

```net
protocol Connection {
    Ping = 0;
    (request: HTTPRequest -> response: HTTPResponse) = 1;
    (Message -> ()) = 2;
}
```

### Messages

- [ ] Syntax Highlight Defined
- [ ] Parser Implemented
- [ ] Parser Tests Implemented
- [ ] Generator Implemented
- [ ] Generator Tests Implemented

Using the `option` keyword before `struct` makes all fields optional. This is the analogous to a `message` in [Bebop](https://github.com/betwixt-labs/bebop)

```net
option struct Struct {
    Field = 1;
}
```

## Examples

The following recursively referenced construct will be analogous to JSON

```
enum Any {
    Null = 0;
    Bool(bool) = 1;
    Number(i(64)) = 2;
    String(string) = 3;
    Array(Any[]) = 4;
    Object(map(string -> Any)) = 5;
}
```
