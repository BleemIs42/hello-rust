fn main() {
    let s1 = String::from("hello");
    println!("{}, world!", s1);
    let s2 = s1;
    // println!("{}, {}, world!", s1, s2);

    let s3 = "test";
    let s4 = s3;
    println!("{}, {}, world!", s3, s4);

    call1(&s2);
    call2(&s2);

    test1();
    test2();

    slice1();
}

fn call1(s: &String){
    println!("{}, world!", s);
}
fn call2(s: &String){
    println!("{}, world!", s);
}

fn test1(){
    let n1 = "简单教程".to_string();
    let n2 = "简单编程".to_string();
 
    let n3 = n1 + &n2; // 需要传递 n2 的引用
    println!("n2: {}",n2);
    println!("n3: {}",n3);
}

fn test2(){
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);
}

fn slice1(){
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}