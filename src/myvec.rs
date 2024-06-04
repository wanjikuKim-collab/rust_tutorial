
use ring::aead::{Aad, BoundKey, Nonce, SealingKey, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use hex;


pub fn test_vec_int(){
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(25);
    my_ints.push(27);
    my_ints.push(29);

    println!("{:?}", my_ints);
    println!("The length of Vec: {:?}", my_ints.len());
    println!("The capacity of Vec: {:?}", my_ints.capacity());
    println!("My first born brother is {:?} years old", &my_ints[2]);

    let v = vec![10,20,30,40,50,60,70,80,90];
    println!("{:?}When rounded of to the nearest hundreds they become 100", &v.as_slice()[4..]);
    println!("Testing the get method of referencing a value {:?}", v.get(10));
    println!("Referencing some value using the get enum: {:?}", v[v.len()-1])

    
}

pub fn test_vec_string(){
    let siblings = vec!["Samuel", "David", "Emmy"];

    // for i in siblings{
    //     println!("Sibling name: {}", i);
    // }

    // println!("Siblings: {:?}", siblings);// This will cause a borrow error

    //solution to the borrow error

    // slice or clone
    for i in siblings.clone(){
        println!("Sibling name: {}", i);
    }

    println!("After slice {:?}", siblings);
}

#[derive(Debug)]
struct Car{
    manufacturer: String,
    model: String,
    cc: u16,
}

// I want to initialize my vec to contain the same values of elements
pub fn manufacture_cars(){
    let porsche_cc = vec![3996; 10];
    println!("First line of porsche engine assign {:?}cc", porsche_cc);

    let mut car_list = Vec::new();
    for i in porsche_cc.as_slice(){
        car_list.push(Car{manufacturer: "Porsche". to_string(),model: "Porsche 911".to_string(), cc: {*i}})
    }

    println!("The manufactured porsche list is: {:#?}", car_list);
    println!("The manufactured porsche vector LENGTH is: {:#?}", car_list.len());
    println!("The manufactured porsche vector CAPACITY is: {:#?}", car_list.capacity());
}


pub fn merge_company(){
    let mut dt_dobie = vec!["VW", "Mercedes", "Hyundai", "Sino Trucks"];
    let mut cfao = vec!["Toyota", "Yamaha", "Prado" ];

    cfao.append(&mut dt_dobie);
    println!("dt_dobie:{:?}", dt_dobie);
    println!("cfao: {:?}", cfao);

    cfao.insert(2, "Cheverolet");
    println!("cfao: {:?}", cfao);

    let vec1 = vec![10,20,30,40];
    let vec2 = vec![vec1];

    println!("This is vec2 {:?}", vec2);
    println!("Goodbye Kimmy");


}
//Create unit struct for the counter nonce sequence
pub struct CounterNonceSequence(u32);

// impl NonceSequence for CounterNonceSequence{
//   // called once for each seal operation
//   fn advance(&mut self)-> Result<Nonce, Unspecified>{
//     let mut nonce_bytes = vec![0; NONCE_LEN];

//     let bytes = self.0.to_be_bytes();
//     nonce_bytes[8..].copy_from_slice(&bytes);
//     println!("nonce_bytes = {}", hex::encode(&nonce_bytes));

//     self.0 += 1; //advance the counter
//     Nonce::try_assume_unique_for_key(&nonce_bytes)
//   }
// }

pub fn encrypt(){
    let rand = SystemRandom::new();

    let mut key_bytes = vec![0;AES_256_GCM.key_len()];
    println!("key_bytes: {:?}", key_bytes);

    rand.fill(&mut key_bytes);
    println!("key_bytes ={}", hex::encode(&key_bytes));// encodes data into hexadecimal representation
}