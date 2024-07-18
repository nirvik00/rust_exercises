use ndarray::arr2;

pub fn run() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);

    let c = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let d = arr2(&[[6, 5], [5, 2], [4, 1]]);
    println!("{:?}", c.dot(&d));

    println!("{:?}", d * 12);
    println!("{:?}", c * 12);
}
