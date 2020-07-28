use std::io;

fn main() {
    // // println!("Hello, world!");
    let condition= false;
    //if with let
    let number = if condition{5} else {7};
    println!("the number is {}", number);
    // returning vlauyes from loops
    let mut count = 0;
    let res = loop{
        if count ==5{
            break count+1;
        }
        else{
            count+=1;
        }
    };
    println!("Count +1 is {}",res);
    //for loop
    let a1= [1,2,3,4];
    for i in a1.iter() {
        println!("num {}",i);
    }
    //for loop with range
    for i in (1..5).rev(){
        println!("{}",i);
    }

    //fibbonacci    
    // loop{
        let mut num = String::new();
        println!("\nenter fib");
        io::stdin().read_line(&mut num).expect("read failed");
        let num: i32= match num.trim().parse() {
        Ok(num)=> num, 
        Err(_)=> -1,
        };
        println!("the first {} fib nums are", num);
        let mut n1 = 0;
        let mut n2 = 1;
        let mut sum:u32;
        print!("{} ",1);
        for _i in 1..num{
            sum= n1+n2;
            print!("{} ",sum);
            n1=n2;
            n2= sum;

        }
    //}

    //C to F
    // loop{
        let mut f=String::new();
        println!("\nenter the temp in F");
        io::stdin().read_line(&mut f).expect("read failed");
        let f : f32=  match f.trim().parse() {
            Ok(f) => f,
            Err(_)=> -1.0,
        };
        let c: f32 = (f-32.)*5./9.;
        println!("\nThe temp in C is {}", c);
    // }
    //Ownership
    let i1= 5;
    let i2=i1;
    println!("{:?}",(i1,i2));
    let s1= String::from("Hello");
    // let s2=s1;
    //doesn't work
    // println!("{:?}", (s1,s2));
    
    //borrowing
    let s = String::from("hello world");
    let res = frist_word(&s);
    println!("first word is {}", res);

}   

fn frist_word(s:&str) -> &str
{
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate()
    {
        if item == b' ' // b' ' is a space in byte code
        {
            return &s[..index];
        }
    }
    &s[..]
}

