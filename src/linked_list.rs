use std::rc::Rc;

#[derive(Eq, PartialEq)]
pub struct LinkedList<T> {
    head: Rc<LinkedListNode<T>>,
    len: usize,
}

#[derive(Eq, PartialEq)]
enum LinkedListNode<T> {
    Empty,
    Cons(T, Rc<Self>),
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: Rc::new(LinkedListNode::<T>::Empty),
            len: 0,
        }
    }

    pub fn head(&self) -> Option<&T> {
        match self.head.as_ref() {
            LinkedListNode::Empty => None,
            LinkedListNode::Cons(item, _) => Some(&item),
        }
    }

    pub fn tail(&self) -> Option<Self> {
        match self.head.as_ref() {
            LinkedListNode::Empty => None,
            LinkedListNode::Cons(_, rest) => Some(LinkedList {
                head: Rc::clone(&rest),
                len: self.len - 1,
            }),
        }
    }

    pub fn push(&self, item: T) -> Self {
        let cons = Rc::new(LinkedListNode::<T>::Cons(item, Rc::clone(&self.head)));
        Self {
            head: cons,
            len: self.len + 1,
        }
    }

    pub fn contains(&self, needle: &T) -> bool
    where
        T: Eq,
    {
        let mut current_node = &self.head;
        while let LinkedListNode::Cons(item, next_node) = current_node.as_ref() {
            if item == needle {
                return true;
            }
            current_node = next_node;
        }
        return false;
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter { head: &self.head }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list = list.push(item);
        }
        list
    }
}

pub struct LinkedListIter<'a, T> {
    head: &'a LinkedListNode<T>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(self: &mut Self) -> Option<&'a T> {
        match self.head {
            LinkedListNode::Empty => None,
            LinkedListNode::Cons(item, tail) => {
                self.head = &tail;
                Some(item)
            }
        }
    }
}

// impl<T> IntoIterator for LinkedList<T> {
//     type Item = T;
//     type IntoIter = LinkedListIter<T>;

//     fn into_iter(self) -> Self::IntoIter {
//         Self::IntoIter { list: self }
//     }
// }

impl<T> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        Self {
            head: Rc::clone(&self.head),
            len: self.len,
        }
    }
}
