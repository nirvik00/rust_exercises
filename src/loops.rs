pub fn run(){
    let mut count=0;
    
    // infinite loop
    loop{
        count+=1;
        println!("num: {}", count);
        if count>=20{
            break;
        }
    }

    // while loop
    let mut i=0;
    while i<100{
        if i%15 ==0 {
            println!("fizzbuzz");
        }else if i%3==0 {
            println!("fizz");
        }else if i%5==0 {
            println!("buzz");
        }else{
            println!("num {}", i);
        }
        i+=1;
    }


    for x in 0..100{
        if x%15 ==0 {
            println!("fizzbuzz");
        }else if x%3==0 {
            println!("fizz");
        }else if x%5==0 {
            println!("buzz");
        }else{
            println!("num {}", x);
        }
    }
}