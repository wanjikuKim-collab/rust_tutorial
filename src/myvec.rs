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