use std::mem;
pub fn run(){
    let mut numbers: Vec<i32>= vec![1,2,3,4];
    //add to vecotr
    numbers.push(5);
    numbers.push(6);
    //pop last value
    numbers.pop();
    // re assign value
    numbers[2]=20;
    println!("{:?}", numbers);
    //get single val
    println!("Single value {}", numbers[0]);
    //get array length
    println!("vector length {}", numbers.len());
    //vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));
    //get slice
    let slice: &[i32]=&numbers[0..2];
    println!("Slice {:?}", slice);
    for x in numbers.iter(){
        println!("Numbers: {}", x);
    }
    for x in numbers.iter_mut(){
        *x*=2;
    }
    println!("Numbers Vec: {:?}", numbers);
}