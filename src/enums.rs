enum Movement{
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m:Movement){
    //perform action depending on info
    match m{
        Movement::Up=> println!("Up"),
        Movement::Down=> println!("Down"),
        Movement::Left=> println!("Left"),
        Movement::Right=> println!("Right")
    }
}

pub fn run(){
    let a1= Movement::Left;
    let a2= Movement::Right;
    let a3= Movement::Up;
    let a4=Movement::Down;

    move_avatar(a1);
    move_avatar(a2);
    move_avatar(a3);
    move_avatar(a4);
}