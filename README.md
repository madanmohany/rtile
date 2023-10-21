# rtile

rtile provides a way to work with rectangular areas of text as atomic units which can be used for code generation

## How to use

```rust
use rtile::prelude::*;
use std::collections::BTreeMap;

fn main() {
    codegen();
}

fn codegen() {
    tp!(
        struct_def,
        "
            #[derive(Default, Debug)]
            @{s_vis}struct @{s_name}{
                @{s_members}
            }
        "
    );
    let mut input: BTreeMap<&str, (bool, Vec<&str>, Vec<&str>)> = BTreeMap::new();
    input.insert(
        "Person",
        (
            true,
            vec!["name", "age", "address", "properties"],
            vec!["String", "u32", "Vec<Address>", "Properties"],
        ),
    );
    input.insert(
        "Address",
        (
            true,
            vec!["street", "city", "state", "zip"],
            vec!["String", "String", "String", "String"],
        ),
    );
    input.insert(
        "Properties",
        (
            true,
            vec!["gender", "kids", "other_details"],
            vec!["Gender", "Option<u8>", "OtherDetails"],
        ),
    );
    let mut struct_codes = vec![];
    for (key, value) in &input {
        if value.0 {
            kp!(s_vis, "pub ");
        } else {
            tp!(s_vis);
        }
        tp!(s_name, "{}", key);
        let val: Vec<_> = value
            .1
            .iter()
            .zip(&value.2)
            .map(|(k, v)| {
                if value.0 {
                    format!("pub {}: {},", k, v)
                } else {
                    format!("{}: {},", k, v)
                }
            })
            .collect();
        tp!(s_members, t!(val));
        struct_codes.push(ts!("@{struct_def}"));
    }
    tp!(
        enum_def,
        "
            #[derive(Debug)]
            @{e_vis}enum @{e_name}{
                @{e_members}
            }
        "
    );
    let mut input: BTreeMap<&str, (bool, Vec<&str>, Vec<&str>)> = BTreeMap::new();
    input.insert(
        "EmploymentStatus",
        (
            true,
            vec![
                "Employed",
                "Unemployed",
                "Employer",
                "Retired",
                "NotApplicable",
            ],
            vec![""; 5],
        ),
    );
    input.insert(
        "Gender",
        (true, vec!["Unknown", "Male", "Female"], vec![""; 3]),
    );
    input.insert(
        "OtherDetails",
        (
            true,
            vec!["Miscellaneous"],
            vec!["{education: Option<String>, employment_status: EmploymentStatus,}"],
        ),
    );
    let mut enum_codes = vec![];
    for (key, value) in &input {
        if value.0 {
            kp!(e_vis, "pub ");
        } else {
            tp!(e_vis);
        }
        tp!(e_name, "{}", key);
        let val: Vec<_> = value
            .1
            .iter()
            .zip(&value.2)
            .map(|(k, v)| format!("{}{},", k, v))
            .collect();
        tp!(e_members, t!(val));
        enum_codes.push(ts!("@{enum_def}"));
    }
    struct_codes
        .into_iter()
        .for_each(|code_item| println!("{code_item}\n"));
    enum_codes
        .into_iter()
        .for_each(|code_item| println!("{code_item}\n"));
}
```

## Formatted output

```rust
#[derive(Default, Debug)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[derive(Default, Debug)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub address: Vec<Address>,
    pub properties: Properties,
}

#[derive(Default, Debug)]
pub struct Properties {
    pub gender: Gender,
    pub kids: Option<u8>,
    pub other_details: OtherDetails,
}

#[derive(Debug)]
pub enum EmploymentStatus {
    Employed,
    Unemployed,
    Employer,
    Retired,
    NotApplicable,
}

#[derive(Debug)]
pub enum Gender {
    Unknown,
    Male,
    Female,
}

#[derive(Debug)]
pub enum OtherDetails {
    Miscellaneous {
        education: Option<String>,
        employment_status: EmploymentStatus,
    },
}
```
