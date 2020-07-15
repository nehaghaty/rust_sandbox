use std::mem;
pub fn run(){
    let mut numbers:[i32;4]=[1,2,3,4];
    println!("{:?}", numbers);
    // re assign value
    numbers[2]=20;
    //get single val
    println!("Single value {}", numbers[0]);
    //get array length
    println!("array length {}", numbers.len());
    //array are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32]=&numbers[0..2];
    println!("Slice {:?}", slice);
    
}