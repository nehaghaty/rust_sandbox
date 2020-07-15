pub fn run(){
    let age:u8= 22;
    let check_id:bool=true;
    if age>=21{
        println!("Bartender: what drink?");
    }
    else if age < 21 && check_id {
        println!("sorry u have to leave");
    }
    else{
        println!("need to see id");
    }
    let is_of_age = if age >=21 {true} else {false};
    println!("is of age {}", is_of_age);
    
}