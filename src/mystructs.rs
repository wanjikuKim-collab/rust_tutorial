#[derive(Debug)]
pub struct Person{
    pub age: u8,
    pub gender: char,
    pub single: bool,
    pub name: String,    
}

pub fn build_person(name:String, age: u8, gender:char, single:bool) -> Person{ 
    Person{
        name,
        age,
        gender,
        single,
    }
}