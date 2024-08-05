pub fn run(){
    let args: Vec<String>  = std::env::args().collect();
    println!("Args {:?}", args);

    let cmd = args[1].clone();
    let name="ns";

    println!("Args {:?}", cmd);

    if cmd == "hello"{
        println!("hi how are you {:?}", name );
    }

}