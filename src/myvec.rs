pub fn test_vec_int(){
    let mut my_ints: Vec<i32> = Vec::new();

    my_ints.push(25);
    my_ints.push(27);
    my_ints.push(29);


    println!("{:?}", my_ints);
    println!("The length of Vec: {:?}", my_ints.len());
    println!("The capacity of Vec: {:?}", my_ints.capacity());
    println!("My first born brother is {:?} years old", my_ints[2]);
    
}