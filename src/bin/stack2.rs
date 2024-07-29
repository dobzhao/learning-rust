//修复堆栈溢出   线程不安全
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
        self.head = Some(Box::new(Node {
            elem: elem,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        println!("Dropping list of {}", std::any::type_name::<T>());
        while let Some(n) = node {
            node = n.next;
        }
    }
}

fn main() {
    let mut list = List::new();
    (0..50_0000).for_each(|x| list.push(x));
    println!("{}", list.pop().unwrap());
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
        drop(list); //如果不写，函数结束才会回收内存
        let mut list = List::new();

        (0..5_0000).for_each(|x| list.push(x));
        assert_eq!(Some(4_9999), list.pop());
        drop(list);
    }
}
