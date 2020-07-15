pub fn run(){
    //default int is i32
    let x=1;
    //default float is f64
    let y=2.5;
    //add explicit type
    let z: i64=454545;
    //find max size
    println!("max i32 is {}", std::i32::MAX);
    println!("max i64 is {}", std::i64::MAX);
    //boolean
    let is_active:bool = true;
    //get boolean from expression
    let is_greater: bool = 10 < 5;
    // char
    let a1:char='b';
    let face= '\u{1F600}'; //unicode is a char
    
    println!("{:?}", (x,y,z,is_active, is_greater, a1, face));
}