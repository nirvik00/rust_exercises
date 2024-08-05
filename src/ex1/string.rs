// str - immutable fixed length 
// String - growable heap-allocated 

pub fn run(){
    let a = "hello";
    let mut b = String::from("He llo");

    println!("{} {}",a.len(), b.len());

    b.push('q');
    println!("{} {}", b, b.len());

    b.push_str(" qqe rty");
    println!("{} {}", b, b.len());

    
    println!("{}", b.capacity());
    println!("{}", b.is_empty());
    println!("{}", b.contains("q ert"));
    println!("{}", b.replace("q ert", "qw erty"));

    for word in b.split_whitespace(){
        println!("{}", word);
    }

    let mut s= String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('c');
    s.push('c');
    s.push('c');
    s.push('c');
    println!("{}", s);

    assert_eq!(6, s.len());
    println!("{}", s);

}