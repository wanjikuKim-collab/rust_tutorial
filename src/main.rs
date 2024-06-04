mod myvec;
mod mystructs;
use mystructs::Person;
// use ring::rand::SystemRandom;

fn main() {
    println!("Hello, Kimmy");

    //vectors
    myvec::test_vec_int();
    myvec::test_vec_string();
    myvec::manufacture_cars();
    myvec::merge_company();
    myvec::encrypt();
    
    // creating my struct instance
    let kimmy = Person{
        age: 25,
        gender: 'F',
        single: true,
        name: String::from("Faith Kimani"),
    };
    println!("{:?}", kimmy);

    let claire = Person{
        name: String::from("Claire Kanina"),
        ..kimmy       
    };
    println!("{:?}", claire);

    let gideon = mystructs::build_person("Gideon".to_string(), 25, 'M', false);
    println!("{:?}", gideon);

    

    let mut input: String = "".to_string();
    std::io::stdin().read_line(&mut input).expect("Something wrong happened");
    
 
}
