struct S {
    num: i32,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("dropping struct S, num: {}", self.num);
    }
}

fn main() {
    let mut a = 3;
    let mut b = a;
    a += 3;
    b += 5;
    println!("{}", a);
    println!("{}", b);

    let a = S { num: 1 };
    let mut b = S { num: 2 };
    println!("b.num = {}", b.num);
    b = a;
    println!("b.num = {}", b.num);
    // println!("{}", a.num);   // compile error

    // Box::leak(Box::new(b));
}
