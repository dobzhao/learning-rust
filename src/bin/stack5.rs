/*
改用Rc，重用元素, 只读，线程不安全
Rc改成Arc目前的功能线程安全，如果要可写得用Mutex或RwLock
*/
use std::rc::Rc;
// use std::sync::Arc;
// use std::sync::Mutex;
type Link<T> = Option<Rc<Node<T>>>;
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

    pub fn prepend(&self, elem: T) -> List<T>{
        List { head: Some(Rc::new(Node{
            elem: elem,
            next: self.head.clone(),
        })) }
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|n| n.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)   //用as_ref避免本身被消费，消费了不可能返回索引
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

/*
只有一个计数的时候，说明是自己生成的元素，drop掉,遇到计数不止1，说明当前list从这里分叉出来的，停止，
随着前一个节点的drop，停止的这个节点的引用计数也会减一
*/
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // println!("drop list of {}", std::any::type_name::<T>());
        let mut next_node = self.head.take();
        while let Some(node) = next_node {
            if let Ok(n) = Rc::try_unwrap(node) { //node被消费
                next_node = n.next;
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.elem
        })
    }
}

fn main() {
    let mut list = List::new();
    for i in 0..50_0000 {
        list = list.prepend(i);
    }
    println!("{}", list.head().unwrap());
}

#[cfg(test)]
mod unit_tests {
    use super::List;
    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
 
        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    } 
    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
