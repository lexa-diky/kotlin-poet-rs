pub mod io;
pub mod spec;

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{ClassLikeTypeName, Import, Name, Package};

    #[test]
    fn it_works() {
        let my_import = Import::class_like(
            ClassLikeTypeName::simple(
                Package::from(
                    vec![
                        Name::from("com"),
                        Name::from("example"),
                    ]
                ),
                Name::from("Foo"),
            )
        );

        println!("{}", my_import.render())
    }
}
