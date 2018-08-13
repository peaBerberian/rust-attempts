use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(| node | node.next.clone())
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(| node | &node.elem)
    }
}

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
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

// The first way is that we can keep grabbing the tail of the list and dropping
// the previous one to decrement its count. This will prevent the old list from
// recursively dropping the rest of the list because we hold an outstanding
// reference to it. This has the unfortunate problem that we traverse the entire
// list whenever we drop it. In particular this means building a list of length
// n in place takes O(n2) as we traverse a lists of length n-1, n-2, .., 1 to
// guard against overflow (this is really really really really bad).
// impl<T> Drop for List<T> {
//     fn drop(&mut self) {
//         // Steal the list's head
//         let mut cur_list = self.head.take();
//         while let Some(node) = cur_list {
//             // Clone the current node's next node.
//             cur_list = node.next.clone();
//             // Node dropped here. If the old node had
//             // refcount 1, then it will be dropped and freed, but it won't
//             // be able to fully recurse and drop its child, because we
//             // hold another Rc to it.
//         }
//     }
// }

// The second way is if we could identify that we're the last list that knows
// about this node, we could in principle actually move the Node out of the Rc.
// Then we could also know when to stop: whenver we can't hoist out the Node.
// For reference, the unstable function is called try_unwrap.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
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
        let list = List::new()
            .append(1)
            .append(2)
            .append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
