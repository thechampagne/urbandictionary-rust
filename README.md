# Urban

[![](https://img.shields.io/github/v/tag/thechampagne/urbandictionary-rust?label=version)](https://github.com/thechampagne/urbandictionary-rust/releases/latest) [![](https://img.shields.io/github/license/thechampagne/urbandictionary-rust)](https://github.com/thechampagne/urbandictionary-rust/blob/main/LICENSE)

Urban Dictionary API client for **Rust**.

### Download
[Crates](https://crates.io/crates/urban/)

Add the following line to your Cargo.toml file:

```
urban = "1.0.0"
```

### Example

```rust
fn main() {
    let response = urban::UrbanDictionary::new("Rust", 1);
    println!("{:?}", response.data().unwrap());
    println!("{:?}", urban::random().unwrap());
    println!("{:?}", urban::definition_by_id(15804563).unwrap());
    println!("{}", urban::tool_tip("Rust").unwrap());
}

```

### License

Urban is released under the [MIT License](https://github.com/thechampagne/urbandictionary-rust/blob/main/LICENSE).