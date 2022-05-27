struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
             x: x,
             y: y
        }
    }

    pub fn set_x(&mut self, v: i32) {
        self.x = v;
    }

    pub fn set_y(&mut self, v: i32) {
        self.y = v;
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn move_in_return_y(self) -> String {
        self.y.to_string()
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("dropping struct S, x: {}, y:{}", self.x, self.y);
    }
}

fn print_point_by_ref(p: &Point) {
    println!("print_point_by_ref, x: {}, y:{}", p.x, p.y);
}

fn print_point(p: Point) {
    println!("print_point, x: {}, y:{}", p.x, p.y);
}

fn main() {
    let mut a = Point::new(0, 0);
    println!("x: {}, y: {}", a.get_x(), a.get_y());
    a.set_x(1);
    a.set_y(2);
    println!("x: {}, y: {}", a.get_x(), a.get_y());
    println!("y: {}", a.move_in_return_y());
    // println!("x: {}, y: {}", a.get_x(), a.get_y());

    let b = Point{x:1, y:100};
    print_point_by_ref(&b);
    print_point_by_ref(&&&b);
    print_point(b);
    // print_point_by_ref(&b);  //编译失败
    // print_point(b);
    println!("end");
}
