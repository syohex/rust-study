struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    fn push(&mut self, elem: T) {
        self.size += 1;
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.size -= 1;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    fn top(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.elem),
        }
    }

    fn top_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    fn is_empty(&self) -> bool {
        match &self.head {
            None => true,
            Some(_) => false,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }
}

struct IntoIter<T>(List<T>);

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &node.elem
        })
    }
}

fn main() {
    let mut lst = List::<i32>::new();
    lst.push(1);
    lst.push(2);
    lst.push(3);
    assert_eq!(lst.pop(), Some(3));
    assert_eq!(lst.pop(), Some(2));
    assert_eq!(lst.pop(), Some(1));
    assert_eq!(lst.pop(), None);

    println!("hello");
}
