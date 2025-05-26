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
// 手动实现 Drop trait，避免递归释放
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(mut node) = self.head.take() {
            self.head = node.next.take();
            // node 被移出作用域后自动 drop
        }
    }
}
fn main() {
    let mut list = List::new();
    (0..500000).for_each(|x| list.push(x));
    println!("{}", list.pop().unwrap());
    for x in (0..499999).rev() {
        assert_eq!(Some(x), list.pop());
    }
    assert_eq!(None, list.pop());
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
        assert_eq!(None, list.pop());
        drop(list); //如果不写，函数结束才会回收内存
        let mut list = List::new();

        (0..500000).for_each(|x| list.push(x));
        assert_eq!(Some(499999), list.pop());
        drop(list);
    }
}
