fn main() {
    /*
    pub const fn as_ref(&self) -> Option<&T> {
        match *self {
            Some(ref x) => Some(x),
            None => None,
        }
    }

    pub const fn as_deref(&self) -> Option<&T::Target>
    where
        T: ~const Deref,
    {
        match self.as_ref() {
            Some(t) => Some(t.deref()),
            None => None,
        }
    }
    */

    let s = 4;
    let x = Some(Box::new(s));   //不用box包装直接Some(4)，as_deref编译不过，i32不支持deref trait
    let _y = x.as_ref();   //Box<i32>  -> &Box<i32>
    let _z = x.as_deref();   // Box<i32>  -> &Box<i32>  -> Box<&i32>    Box的deref方法接收&self返回&T

    let s = 4;
    let x = Some(&s);    
    let _y = x.as_ref();    //Option包装的是&i32取引用 &&i32
    let _z = x.as_deref();    // &i32 -> &&i32 -> &i32

    let s = "aaa".to_string();
    let x = Some(s);            //Some(String)
    let _y = x.as_ref();        //Some(&String)
    let _z = x.as_deref();    //Some(&str)  String的deref方法接收&self返回&str


}