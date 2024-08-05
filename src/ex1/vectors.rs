pub fn run(){
    let mut nums: Vec<i32> = vec![1,2,3,4,5]; // define a vector

    println!("{:?}", nums);
    println!("{:?}", nums[0]);

    nums.push(5);
    nums.push(5);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);


    let r:&[i32]=&nums[0..3];
    println!("{:?}", r);

    for x in nums.iter(){
        println!("num: {}", x);
    }


    for x in nums.iter_mut(){
        *x*=23;
    }
    println!("num: {:?}", nums);

}

