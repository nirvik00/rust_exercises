//
//
pub fn run() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", nums);
    println!("{:?}", nums[0]);

    let mut n = [2, 3, 4, 5, 6];
    n[2] = 12;
    println!("{:?}", n);

    println!("{:?}", n.len());
    println!("{:?}", std::mem::size_of_val(&n));

    let m: &[i32] = &n;
    println!("{:?}", m);
    let r: &[i32] = &n[0..3];
    println!("{:?}", r);
}
