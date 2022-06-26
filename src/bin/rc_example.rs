use std::rc::Rc;

struct Point {
    x: i32,
    y: i32
}
impl Drop for Point {
    fn drop(&mut self) {
        println!("drop Point with x = {}", self.x);
    }
}
struct S1 {
    start: Option<Rc<Point>>,
}

impl S1 {
    fn get_s2(&self) -> S2{
        S2 { start: self.start.clone()}
        // S2 { start: self.start}   //编译不过，借用的self，移动它里面的东西的所有权是不被允许的
    }
}
impl Drop for S1 {
    fn drop(&mut self) {
        println!("drop S1");
    }
}
struct S2 {
    start: Option<Rc<Point>>,
}
impl Drop for S2 {
    fn drop(&mut self) {
        println!("drop S2");
    }
}

fn main() {
    let point = Point{x:1, y:1};
    let rc1 = Rc::new(point);
    let rc2 = rc1.clone();
    // println!("{}", point.x);  //编译不过，所有权已经在Rc::new的时候move进去了
    drop(rc1);
    println!("rc2.x: {}", rc2.x);
    drop(rc2);

    let s1 = S1 {
        start: Some(Rc::new(Point{x:3, y:5})),
    };
    let s2 = s1.get_s2();
    drop(s1);
    if let Some(ref p) = &s2.start {
        //如果加上S2的Drop Trait实现，模式匹配必须用引用，否则编译不过，提示发生move，
        //如果没有定义，后面不再借用s2，就可以let Some(p) = s2.start
        //&比.优先级低，*比.优先级高
        println!("x: {}, y: {}", p.x, p.y);
    }
}
