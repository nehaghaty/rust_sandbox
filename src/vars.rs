//vairables
pub fn run(){
    let name="Neha";
    //println!("My name is {}", name);
    let mut age= 35;
    println!("My name is {} and I am {}", name, age);
    age=36;
    println!("My name is {} and I am {}", name, age);
    //define constant
    const ID: i32 = 001; // need to set type for const
    println!("ID: {}",ID);
    //assign multiple vars
    let (my_name, my_age)= ("Neha", 23);
    println!("{} is {}", my_name, my_age);
}