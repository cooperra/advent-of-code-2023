use std::rc::Rc;

#[derive(Eq, PartialEq)]
pub struct LinkedList<T> {
    head: Rc<LinkedListNode<T>>,
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
        }
    }

    pub fn head(&self) -> Option<&T> {
        match self.head.as_ref() {
            LinkedListNode::Empty => None,
            LinkedListNode::Cons(item, _) => Some(&item),
        }
    }

    pub fn push(&self, item: T) -> Self {
        let cons = Rc::new(LinkedListNode::<T>::Cons(item, Rc::clone(&self.head)));
        Self { head: cons }
    }

    pub fn contains(&self, needle: &T) -> bool
    where
        T: Eq,
    {
        let next_node = &self.head;
        while let LinkedListNode::Cons(item, next_node) = next_node.as_ref() {
            if item == needle {
                return true;
            }
        }
        return false;
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
