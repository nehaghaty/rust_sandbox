pub fn run(){
    let mut hello=String::from("Hello ");
    //get length
    println!("{}",hello.len());
    hello.push('W');
    hello.push_str("orld");
    println!("{}",hello);
    //capacity in bytes
    println!("capacity {}", hello.capacity());
    //check if empty
    println!("is empty {}", hello.is_empty());
    //cotains
    println!("constians 'World' {}", hello.contains("World"));
    //replace
    println!("replace: {}", hello.replace("World", "there"));
    //loop through string using whitespace
    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    //create strinng with capacity
    let mut s =String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion
    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());
    println!("{}",s);

}