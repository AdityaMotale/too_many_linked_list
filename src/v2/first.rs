use std::mem;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Some(Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::None),
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_head = mem::replace(&mut self.head, Link::None);

        while let Some(mut old_node) = cur_head {
            cur_head = mem::replace(&mut old_node.next, Link::None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i8>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);

        list.push(1);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
