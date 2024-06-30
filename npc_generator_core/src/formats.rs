use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use gluon::{vm::api::FunctionRef, vm::primitives, ThreadExt};
use serde::{Deserialize, Serialize};

use crate::AgeRange;

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

use lazy_static::lazy_static;
lazy_static! {
    static ref gluon_vm: Arc<Mutex<gluon::RootedThread>> = {
        let vm = gluon::new_vm();

        let source = gluon::vm::api::typ::make_source::<AgeRange>(&vm).unwrap();

        vm.load_script("npc_generator.core", &source).unwrap();
        Arc::new(Mutex::new(vm))
    };
}
fn create_format_vm() -> Arc<Mutex<gluon::RootedThread>> {
    gluon_vm.clone()
}
impl Formats {
    pub fn format_full_name<'a>(
        &self,
        first_name: &'a str,
        surname: &'a str,
        additional_names: Vec<&'a str>,
    ) -> String {
        let vm = create_format_vm();
        let (mut function, _) = vm.lock().unwrap()
            .run_expr::<gluon::vm::api::OwnedFunction<fn(&'a str, &'a str, Vec<&'a str>) -> String>>(
                "formatter",
                &self.full_name.0
            )
            .unwrap();

        function
            .call(first_name, surname, additional_names)
            .unwrap()
    }

    pub fn format_flavor_description_line<'a>(
        &self,
        default: &str,
        name: &'a str,
        age: u64,
        age_range: crate::AgeRange,
        sex: &'a str,
        ancestry_name: &'a str,
        heritage_name: &'a str,
        job_name: &'a str,
    ) -> String {
        let vm = create_format_vm();

        let (mut function, _) = vm
            .lock()
            .unwrap()
            .run_expr::<gluon::vm::api::OwnedFunction<
                fn(&'a str, u64, crate::AgeRange, &'a str, &'a str, &'a str, &'a str) -> String,
            >>("formatter", default)
            .unwrap();

        function
            .call(
                name,
                age,
                age_range,
                sex,
                ancestry_name,
                heritage_name,
                job_name,
            )
            .unwrap()
    }
}

impl HeritageFormats {
    pub fn format_lineage_line<'a>(&self, lineage: &'a str) -> String {
        let vm = create_format_vm();

        let (mut function, _) = vm
            .lock()
            .unwrap()
            .run_expr::<gluon::vm::api::OwnedFunction<fn(&'a str) -> String>>(
                "formatter",
                &self.lineage_line.0,
            )
            .unwrap();

        function.call(lineage).unwrap()
    }
}
