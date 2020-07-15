pub fn run(){
    //print to console
    println!("hello from the print.rs file");
    //basic formatting
    println!("{} is from {}", "Neha", "Blr");
    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Neha","Blr", "code");
    //names arguments
    println!("{name} likes to play {activity}", name="John", activity="baseball");
    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);
    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    //basic math
    println!("10+10 = {}", 10+10);
}