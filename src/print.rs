pub fn run(){
    println!("Hello from print rs file");
    println!("{} from {}", "hello", "ns");
    println!("{0} from {1} by {0}", "ns", "mass"); // positional args
    println!("{name} likes {name2}",name="ns", name2="dc" ); // named args
    println!("{:?}",(12, "ns", true) ); // debug args
    println!("10+10={}", 10+10 ); // debug args
}