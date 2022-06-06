// 增加迭代器

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct  Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {   //隐式的执行了Copy
            // self.next = n.next.as_ref().map(|n| &**n);
            self.next = n.next.as_deref();
            &n.elem
        })
    }
}

pub struct  IterMut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.elem
        })
    }
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

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|n| &mut n.elem)
    }
    
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        // Iter { next: self.head.as_ref().map(|h| &(**h))}
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        // println!("Dropping list of {}", std::any::type_name::<T>());
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        for i in 1..10 {
            list.push(i.to_string());
        }
        let mut iter = list.into_iter();
        assert_eq!(iter.next().unwrap(), "9".to_string());
        assert_eq!(iter.next().unwrap(), "8".to_string());
        // list.peek();   //编译不过，into_iter()所有权已经转移到IntoIter里去了
        let mut i = 7;
        while let Some(x) = iter.next() {
            assert_eq!(x, i.to_string());
            i-=1;
        }
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        for i in 1..10 {
            list.push(i.to_string());
        }
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap(), "9");
        assert_eq!(iter.next().unwrap(), "8");

        let mut i = 7;
        while let Some(x) = iter.next() {
            assert_eq!(x, i.to_string().as_str());
            i-=1;
        }
        assert_eq!(list.pop(), Some("9".to_string()));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        for i in 1..10 {
            list.push(i.to_string());
        }
        let mut iter = list.iter_mut();
        assert_eq!(iter.next().unwrap(), "9");
        assert_eq!(iter.next().unwrap(), "8");

        let mut i = 7;
        while let Some(x) = iter.next() {
            assert_eq!(x, i.to_string().as_str());
            i-=1;
        }
        assert_eq!(list.pop(), Some("9".to_string()));
        let mut iter = list.iter_mut();
        iter.next().unwrap().insert_str(1, "A"); //修改一下第一个元素
        assert_eq!(list.pop(), Some("8A".to_string()));
        assert_eq!(list.pop(), Some("7".to_string()));
    }
}
