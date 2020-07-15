pub fn run(){
    greeting("hello", "jane");
    
    //bind function values to variables
    let get_sum = add(5,5);
    println!("sum {}", get_sum);
}

fn greeting(greet: &str, name: &str){
println!("{} {}",greet,name);
let n3:i32 = 10;
let add_nums = |n1:i32, n2:i32| n1+n2+n3;
println!("C sum {}", add_nums(2,3));

}

fn add(n1:i32, n2: i32) -> i32{
    n1+n2
}