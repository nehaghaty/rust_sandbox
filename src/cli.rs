use std::env;
pub fn run(){
    let args:Vec<String>= env::args().collect();
    let command= args[1].clone();
    println!("Command {:?}", command);
    let name = "Neha";
    let status="90%";
    if command=="hello"{
        println!("Hi {}, how are you?", name);
    }
    else if command=="status"{
        println!("Status is {}", status);
    }
    else{
        println!("That is not a valid commmand");
    }
}