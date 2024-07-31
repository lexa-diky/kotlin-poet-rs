# Roadmap

## Road to 1.0

Currently library is in heavy development.
This roadmap might change daily, contracts will be broken.
Please, don't use this library in production.

There is currently no estimate when `1.0` release will come.

## Planned features

### High priority

- Annotations

### Low priority

- Context parameters
- `CodeBlock` builders
- `spec::Module`
- Standard library `Type` factories generation
- Parsing of some `spec` types from strings

### Researching

- Linting of build specs to ensure that generated code is semantically correct.
- Code formatting
- Non-copy `CodeBlock` builders

## Implemented features

### Typealias

- ✅ Name
- ✅ Associated type (via `Type`)
- ✅ Generic parameters
- ✅ Visibility modifiers
- ✅ Rendering

### Package

- ✅ Root package
- ✅ Nested packages
- ✅ Rendering

### Name

- ✅ Simple Java compatible name
- ✅ Escaped UTF-8 name
- ✅ String parsing
- ✅ Rendering

### Import

- ✅ Class / Interface / Object imports
- ✅ Whole package imports
- ✅ Top level function / property imports
- ✅ Import aliases
- ✅ Rendering

### ClassLikeTypeName

- ✅ Top level class names
- ✅ Nested class names
- ✅ Rendering

### Argument

- ✅ Named arguments
- ✅ Positional arguments
- ✅ Rendering

### Annotation
- ✅ Annotation on entity
- ✅ Annotation arguments
- ✅ Annotation with target specifier
- ✅ Rendering

### Comment
- ✅ Inline and block comments
- ✅ Rendering

### KDoc
- ✅ Merging multiple KDocs into one
- ✅ Rendering

### _TBD_