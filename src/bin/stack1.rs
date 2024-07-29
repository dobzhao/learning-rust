//初步功能，数据量大会出问题，线程不安全

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
        // let node = self.head.take();
        // match node {
        //     Some(n) => {
        //         self.head = n.next;
        //         Some(n.elem)
        //     }
        //     None => None
        // }

        // if let Some(n) = node {
        //     self.head = n.next;
        //     Some(n.elem)
        // } else {
        //     None
        // }

        self.head.take().map(|n| {
            self.head = n.next;
            n.elem
        })
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

        let mut list = List::new();

        // for i in 0..5_0000 {
        //     list.push(i);
        // }

        (0..500).for_each(|x| list.push(x));
        assert_eq!(Some(499), list.pop());
        // DOTO: test里5万条会栈溢出, main里5万条不会，主线程默认8M，test给多少？
        //(0..5_0000).for_each(|x| list.push(x));
        //assert_eq!(Some(4_9999), list.pop());

        // let mut list = std::collections::LinkedList::new();
        // (0..1000_0000).for_each(|x| list.push_front(x));
        // assert_eq!(Some(999_9999), list.pop_front());
    }
}
