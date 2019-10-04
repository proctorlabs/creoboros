use super::{CONTEXT, TEMPLAR};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
use templar::{Document, Template};

macro_rules! impl_render {
    ( $( $name:ident ( $method:ident ) ; )* ) => {
        $(
            #[derive(Debug, Clone, Default)]
            pub struct $name(Template, String);

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.1)
                }
            }

            #[allow(dead_code)]
            impl $name {
                pub fn exec(&self) -> crate::Result<Document> {
                    self.0
                        .exec(&*CONTEXT)
                        .map_err(|e| crate::error::AppError::Critical{message: e.to_string()})
                }

                pub fn render(&self) -> crate::Result<String> {
                    self.0
                        .render(&*CONTEXT)
                        .map_err(|e| crate::error::AppError::Critical{message: e.to_string()})
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str(&self.1)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    struct ExpressionVisitor;

                    impl<'de> Visitor<'de> for ExpressionVisitor {
                        type Value = $name;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("string")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where
                            E: de::Error,
                        {
                            let template: Template = TEMPLAR.$method(value).unwrap_or_default();
                            Ok($name(template, value.into()))
                        }
                    }

                    deserializer.deserialize_str(ExpressionVisitor)
                }
            }

            impl From<&str> for $name {
                fn from(s: &str) -> $name {
                    let template: Template = TEMPLAR.$method(s).unwrap_or_default();
                    $name(template, s.into())
                }
            }
        )*
    };
}

impl_render! {
    ConfigExpression(parse_expression);
    ConfigTemplate(parse);
}
