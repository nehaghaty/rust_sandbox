#[derive(Debug)]
struct Rect{
    height:u32,
    width:u32,
}

impl Rect {
    fn area(&self)-> u32{
        self.height*self.width
    }
}
impl Rect{
    fn can_hold(&self, r: &Rect)-> bool{
        if self.width > r.width && self.height > r.height{
            return true;
        }
        false
    }
    
}



fn main() {
    // println!("Hello, world!");
    let r1= Rect{
        height: 30,
        width: 40
    };  

    let r2= Rect{
        height:30,
        width: 10,
    };
    println!("area is {}", r1.area());  
    let mut can= String::from("can");
    if !(r1.can_hold(&r2)) {
        can="cannot".to_string();
    }
    println!("r1 {} hold r2",can);
    println!("struct is {:#?}", r2);
}
