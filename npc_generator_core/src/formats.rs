use std::fmt::Display;

use gluon::{vm::api::FunctionRef, vm::primitives, ThreadExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct FormatString(String);

impl FormatString {
    fn new() -> Self {
        Self(String::new())
    }

    fn into_string(self) -> String {
        self.0
    }
}

impl Display for FormatString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for FormatString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for FormatString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for FormatString {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Formats {
    pub full_name: FormatString,
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct HeritageFormats {
    pub lineage_line: FormatString,
}
impl Default for Formats {
    fn default() -> Self {
        Self {
            full_name: r#"\first_name surname additional_names -> first_name ++ " " ++ surname"#
                .into(),
        }
    }
}

impl Default for HeritageFormats {
    fn default() -> Self {
        Self {
            lineage_line: r#"\lineage -> "They are of the " ++ lineage ++ " lineage.""#.into(),
        }
    }
}

impl Formats {
    pub fn format_full_name<'a>(
        &self,
        first_name: &'a str,
        surname: &'a str,
        additional_names: Vec<&'a str>,
    ) -> String {
        let vm = gluon::new_vm();

        let (mut function, _) = vm
            .run_expr::<gluon::vm::api::OwnedFunction<fn(&'a str, &'a str, Vec<&'a str>) -> String>>(
                "format_full_name",
                &self.full_name.0
                //r#"\first_name surname additional_names -> first_name ++ " " ++ surname"#,
            )
            .unwrap();

        function
            .call(first_name, surname, additional_names)
            .unwrap()
    }
}

impl HeritageFormats {
    pub fn format_lineage_line<'a>(&self, lineage: &'a str) -> String {
        let vm = gluon::new_vm();

        let (mut function, _) = vm
            .run_expr::<gluon::vm::api::OwnedFunction<fn(&'a str) -> String>>(
                "format_lineage_line",
                &self.lineage_line.0,
            )
            .unwrap();

        function.call(lineage).unwrap()
    }
}
