//增加peek, peet_mut方法


type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Node {
            elem: elem,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)   //用as_ref避免本身被消费，消费了不可能返回索引
    }

    // pub fn peek(&self) -> Option<&T> {  //另一种写法
    //     if let Some(ref n) = self.head {  //这样写比较清楚
    //         return Some(&n.elem);     //编译器自动解引用，实际是Some(&(*n).elem)
    //     }
    //     // if let Some(n) = &self.head { //这样写让人迷惑，但是编译器这样建议，确实能工作
    //     //     return Some(&n.elem);
    //     // }
    //     None
    // }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }
    
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        println!("Dropping list of {}", std::any::type_name::<T>());
        while let Some(mut n) = node {
            node = n.next.take();
        }
    }
}

fn main() {
    let mut list = List::new();
    (0..50_0000).for_each(|x| list.push(x));
}

#[cfg(test)]
mod unit_tests {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(String::from("a"));
        list.push(String::from("b"));
        list.push(String::from("c"));
        assert_eq!(Some("c".to_string()), list.pop());
        assert_eq!(Some("b".to_string()), list.pop());
        assert_eq!(Some("a".to_string()), list.pop());
        drop(list);  //如果不写，函数结束才会回收内存
        let mut list = List::new();
        (0..5_0000).for_each(|x| list.push(x));
        assert_eq!(Some(4_9999), list.pop());
        drop(list);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        (0..10).for_each(|x| list.push(x));
        if let Some(&x) = list.peek() {
            assert_eq!(9, x);
        }
        assert_eq!(Some(9), list.pop());
        if let Some(x) = list.peek_mut() {
            *x = -1;
        }
        assert_eq!(Some(-1), list.pop());
        drop(list);

        let mut list = List::new();
        (0..10).for_each(|x| list.push(x.to_string()));
        if let Some(x) = list.peek() {
            assert_eq!(String::from("9"), *x);
        } else {
            panic!("empty list")
        }
    }
}
