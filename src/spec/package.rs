use std::path::PathBuf;
use crate::io::RenderKotlin;
use crate::spec::{CodeBlock, Name};
use crate::tokens::DOT;
use crate::util::{yolo_from_str, SemanticConversionError};
use std::str::FromStr;

/// Fully qualified package name, may be parsed from [&str]
///
/// # Examples
///
/// ## Parse from string
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::Package;
///
/// let package = Package::from_str("io.github.lexadiky").unwrap();
///
/// assert_eq!(
///     package.render_string(),
///     "io.github.lexadiky"
/// );
/// ```
///
/// ## Create from [Vec<Name>]
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{Name, Package};
///
/// let package = Package::from(vec![
///     Name::from("io"),
///     Name::from("github"),
///     Name::from("lexadiky"),
/// ]);
///
/// assert_eq!(
///     package.render_string(),
///     "io.github.lexadiky"
/// );
/// ```
///
/// ## Create root
/// ```rust
/// use std::str::FromStr;
/// use kotlin_poet_rs::io::RenderKotlin;
/// use kotlin_poet_rs::spec::{Name, Package};
///
/// let package = Package::root();
///
/// assert_eq!(
///     package.render_string(),
///     ""
/// );
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    pub(crate) parts: Vec<Name>,
}

impl Package {
    /// Creates new package from [Name] parts
    pub fn from(names: Vec<Name>) -> Package {
        Package { parts: names }
    }

    /// Create root package
    pub fn root() -> Package {
        Package { parts: Vec::new() }
    }

    /// Converts package to Java-like folder structure path
    #[cfg(feature = "experimental")]
    pub fn to_path(&self) -> PathBuf {
        let mut buf = PathBuf::new();
        for part in &self.parts {
            let part_str: String = part.clone().into();
            buf.push(part_str)
        }

        buf
    }

    pub(crate) fn is_root(&self) -> bool {
        self.parts.is_empty()
    }
}

yolo_from_str!(Package);
impl FromStr for Package {
    type Err = SemanticConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(DOT)
            .filter(|part| !part.is_empty())
            .map(Name::from_str)
            .collect::<Result<Vec<_>, SemanticConversionError>>()?;
        Ok(Package::from(parts))
    }
}

impl RenderKotlin for Package {
    fn render_into(&self, block: &mut CodeBlock) {
        for (index, part) in self.parts.iter().enumerate() {
            block.push_renderable(part);
            if index != self.parts.len() - 1 {
                block.push_atom(DOT);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::RenderKotlin;
    use crate::spec::{Name, Package};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn parse_package() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.parts, vec![
            Name::from_str("io").unwrap(),
            Name::from_str("github").unwrap(),
            Name::from_str("lexadiky").unwrap(),
        ]);
    }

    #[test]
    fn parse_empty_package() {
        let package: Package = "".parse().unwrap();
        assert_eq!(package.parts, vec![]);
    }

    #[test]
    fn render_kotlin() {
        let package: Package = "io.github.lexadiky".parse().unwrap();
        assert_eq!(package.render_string(), "io.github.lexadiky");
    }

    #[test]
    fn render_empty() {
        let package: Package = Package::from(vec![]);
        assert_eq!(package.render_string(), "");
    }

    #[test]
    #[cfg(feature = "experimental")]
    fn test_path_conversion() {
        let package = Package::from_str("a.b.c");
        let expected_path = PathBuf::from_str("a/b/c");
        assert_eq!(
            package.unwrap().to_path(),
            expected_path.unwrap()
        )
    }

    #[test]
    #[cfg(feature = "experimental")]
    fn test_path_empty_conversion() {
        let package = Package::root();
        let expected_path = PathBuf::from_str("");
        assert_eq!(
            package.to_path(),
            expected_path.unwrap()
        )
    }
}
