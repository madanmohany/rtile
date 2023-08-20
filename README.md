# rtile

rtile provides a way to work with rectangular areas of text as atomic units which can be used for code generation

## How to use

```rust
use rtile::*;
use std::collections::BTreeMap;

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
    input.insert("Point", (false, vec!["x", "y"], vec!["f32", "f32"]));
    input.insert(
        "Rectangle",
        (
            false,
            vec!["top_left", "bottom_right"],
            vec!["Point", "Point"],
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
            .collect::<Vec<_>>()
            .iter()
            .map(|(k, v)| {
                if value.0 {
                    format!("pub {}: {},", k, v)
                } else {
                    format!("{}: {},", k, v)
                }
            })
            .collect();
        tp!(s_members, t!(val));
        struct_codes.push(k!(gtp!(struct_def).unwrap()));
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
            .collect::<Vec<_>>()
            .iter()
            .map(|(k, v)| format!("{}{},", k, v))
            .collect();
        tp!(e_members, t!(val));
        enum_codes.push(k!(gtp!(enum_def).unwrap()));
    }
    let impls_default_enums = t!(r#"
        impl Default for Gender{
            fn default()->Self{
                Gender::Unknown
            }
        }
        impl Default for EmploymentStatus{
            fn default()->Self{
                EmploymentStatus::NotApplicable
            }
        }
        impl Default for OtherDetails{
            fn default()->Self{
                Self::Miscellaneous{
                    education: None, 
                    employment_status: EmploymentStatus::default(),
                }
            }
        }
    "#);

    tp!(
        some_functions,
        r#"
            fn print_default_person(){
                println!("{:#?}",Person::default());
            }
        "#
    );
    tp!(
        main_function,
        r#"
            fn main(){
                print_default_person();
                /*
                Person {
                    name: "",
                    age: 0,
                    address: [],
                    properties: Properties {
                        gender: Unknown,
                        kids: None,
                        other_details: Miscellaneous {
                            education: None,
                            employment_status: NotApplicable,
                        },
                    },
                }
                */
            }
        "#
    );
    struct_codes
    .into_iter()
    .for_each(|code_item| println!("{code_item}\n"));
    enum_codes
        .into_iter()
        .for_each(|code_item| println!("{code_item}\n"));
    println!("{impls_default_enums}\n");
    println!(
        "{}",
        t!("
            @{some_functions}

            @{main_function}
        ")
    );
}

fn main() {
    codegen();
}
```

## Output

```rust
#[derive(Default, Debug)]
pub struct Address{
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

#[derive(Default, Debug)]
pub struct Person{
    pub name: String,
    pub age: u32,
    pub address: Vec<Address>,
    pub properties: Properties,
}

#[derive(Default, Debug)]
struct Point{
    x: f32,
    y: f32,
}

#[derive(Default, Debug)]
pub struct Properties{
    pub gender: Gender,
    pub kids: Option<u8>,
    pub other_details: OtherDetails,
}

#[derive(Default, Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug)]
pub enum EmploymentStatus{
    Employed,
    Unemployed,
    Employer,
    Retired,
    NotApplicable,
}

#[derive(Debug)]
pub enum Gender{
    Unknown,
    Male,
    Female,
}

#[derive(Debug)]
pub enum OtherDetails{
    Miscellaneous{education: Option<String>, employment_status: EmploymentStatus,},
}

impl Default for Gender{
    fn default()->Self{
        Gender::Unknown
    }
}
impl Default for EmploymentStatus{
    fn default()->Self{
        EmploymentStatus::NotApplicable
    }
}
impl Default for OtherDetails{
    fn default()->Self{
        Self::Miscellaneous{
            education: None,
            employment_status: EmploymentStatus::default(),
        }
    }
}

fn print_default_person(){
    println!("{:#?}",Person::default());
}

fn main(){
    print_default_person();
    /*
    Person {
        name: "",
        age: 0,
        address: [],
        properties: Properties {
            gender: Unknown,
            kids: None,
            other_details: Miscellaneous {
                education: None,
                employment_status: NotApplicable,
            },
        },
    }
    */
}
```
