mod myvec;

fn main() {
    println!("Hello, Kimmy");
    //vectors
    myvec::test_vec_int();
    myvec::test_vec_string();
    myvec::manufacture_cars();
    myvec::merge_company();

    let mut input: String = "".to_string();
    std::io::stdin().read_line(&mut input).expect("Something wrong happened");
}
