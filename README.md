# KotlinPoet-RS

![docs.rs](https://img.shields.io/docsrs/kotlin-poet-rs?link=https%3A%2F%2Fdocs.rs%2Fkotlin-poet-rs%2Flatest%2Fkotlin_poet_rs%2F)
![Crates.io Total Downloads](https://img.shields.io/crates/v/kotlin-poet-rs?label=version)
![Crates.io Total Downloads](https://img.shields.io/crates/d/kotlin-poet-rs?logo=rust&label=crates.io%20downloads)
![GitHub top language](https://img.shields.io/github/languages/top/lexa-diky/kotlin-poet-rs?logo=rust)

![Crates.io License](https://img.shields.io/crates/l/kotlin-poet-rs?logo=apache)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/lexa-diky/kotlin-poet-rs?logo=github)

> Currently, this project is in heavy development and not ready for any serious use.
> **API might be extremely unstable.**

## Get Started

### Add dependency

```shell
cargo add kotlin-poet-rs
```

### Read documentation

- Stable version: [docs.rs](https://docs.rs/kotlin-poet-rs/latest/kotlin_poet_rs/)
- Most recent: [github.io](https://lexa-diky.github.io/kotlin-poet-rs/kotlin_poet_rs/index.html)

### Start writing poetry

Constant property with initializer:

```rust
use kotlin_poet_rs::spec::{CodeBlock, Name, Property, Type};
use kotlin_poet_rs::io::RenderKotlin;

fn main() {
    let property = Property::new(Name::from("name"), Type::string())
        .constant(true)
        .initializer(CodeBlock::atom("\"Alex\""));

    println!("{}", property.render_string());
}
```

Will print:

```kotlin
public final const val name: kotlin.String = "Alex"
```

## Philosophy

### Explicit better then implicit

RustPoet will not generate code that you did not asked it to generate.
No magic import resolution, no magic type inference, no magic anything.

For example, as you can see in example in `Get Started` section generated code will have
explicit fully qualified type name for `name` property.
Same principle applies redundant for `public` and `final` keywords.

### Writer not reader

RustPoet is designed to be used by humans to generate code that humans can read.
Not for describing Kotlin code in some abstract way.
This library is write only, don't use it to represent and manipulate Kotlin code.

### KotlinPoet / JavaPoet has issues

This project acknowledges that original KotlinPoet and JavaPoet has some issues.
It aims to fix them and provide better API for generating Kotlin code. We don't strife to provide exact same API.

#### No built-in IO

Rust IO is very versatile and comes in many shapes. Thus this project does not suggest any IO implementation.

### No dependencies

This library is commited to be as little as possible.
It does not depend on any other library, except for `std`.

## Supported features

### Typealias

- ✅ Name
- ✅ Associated type (via `Type`)
- ✅ Generic parameters
- ✅ Visibility modifiers

### _TBD_