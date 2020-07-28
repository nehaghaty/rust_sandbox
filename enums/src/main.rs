#![feature(option_result_contains)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}
fn cents(coin: Coin){
    //match - exhaustive
    // match coin {
        // Coin::Penny(coin_str) => 1,
        // Coin::Dime(coin_str) => {
        //     println!("{}",coin_str);
        //     10
        // },
        // Coin::Nickel(coin_str)=> 5,
        // Coin::Quarter(coin_str) => 25,
    //if let - avoid bolier plate code
    if let Coin::Quarter(state) = coin {
        println!("the state the quarter was made in is {:?}", state)
    }
    else{
        println!("Not alabama");
    }
}

fn plus_one(i:Option<i32>)->Option<i32>{
    match i{
        Some(i)=> Some(i+1),
        None => None,
    }
}
fn main() {
    // println!("Hello, world!");
    //Options enum
    let x = Option::Some(2);
    let y= Option::<u32>::None;
    assert_eq!(x.is_some(),true);
    assert_eq!(y.is_none(),true);
    assert_eq!(x.contains(&2),true);
    // let coin_str= Coin::Quarter("Alabama".to_string());
    let coin_str=Coin::Dime;
    cents(coin_str);
    println!("x +1 is {:?}", plus_one(Some(2)));

    
}
