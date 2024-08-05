
enum Movement{
    up,
    dn, 
    le, 
    rt
}

fn move_avatar(m: Movement){
    match m{
        Movement::up => println!("move up"),
        Movement::dn => println!("move dn"),
        Movement::le => println!("move le"),
        Movement::rt => println!("move rt"),
    }
}

pub fn run(){
    let avatar1 = Movement::le;
    let avatar2 = Movement::dn;
    let avatar3 = Movement::up;
    let avatar4 = Movement::rt;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}