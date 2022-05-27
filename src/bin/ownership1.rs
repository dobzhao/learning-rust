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
    b = a;   //这时b原本占用的内存被回收
    println!("b.num = {}", b.num);
    // println!("{}", a.num);   // 编译错误，数据的所有权被转移到b，a不被允许再被使用
    let b = S { num: 3 };  //用let覆盖b，并不会马上回收原本的内存，而是到函数结束
    drop(b);

    // Box::leak(Box::new(b));     //即使自定义的drop方法不做回收内存的事情，内存实际也被回收
}
