
use serde::Deserialize;

#[derive(Debug)]
pub struct MyState {
    #[allow(dead_code)]
    pub value: u64,
    #[allow(dead_code)]
    pub label: String,
}

#[derive(Deserialize)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}


#[derive(Deserialize)]
pub struct InlinePerson<'a>(pub &'a str, pub u8);


#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}
