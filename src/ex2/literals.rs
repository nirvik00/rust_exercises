//
//

fn reverse(pair:(i32, bool)) -> (bool, i32){
    let (i, b) = pair;
    (b, i)
}


pub fn run(){

    println!("one million is {}", 1_000_000u32);

    let vec = vec![1,2,3,45];
    for (i, v) in vec.iter().enumerate(){
        println!("{} {}", i, v);
    }

    let t = (1, false);
    println!("{:?}", t);
    let u= reverse(t);
    println!("{:?}", u);

    let mut counter=0;
    let res = loop{
        counter+=1;
        if counter==10{
            break counter*6;
        }
    };
    println!("{:?}", res);

}