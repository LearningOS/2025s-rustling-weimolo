use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();

        let mut a_current = list_a.start.take();
        let mut b_current = list_b.start.take();

        // Merge nodes by comparing values
        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            let a_val = unsafe { &(*a_ptr.as_ptr()).val };
            let b_val = unsafe { &(*b_ptr.as_ptr()).val };

            if a_val <= b_val {
                let next_a = unsafe { (*a_ptr.as_ptr()).next };
                unsafe { (*a_ptr.as_ptr()).next = None };
                merged_list.add_node(a_ptr);
                a_current = next_a;
                b_current = Some(b_ptr);
            } else {
                let next_b = unsafe { (*b_ptr.as_ptr()).next };
                unsafe { (*b_ptr.as_ptr()).next = None };
                merged_list.add_node(b_ptr);
                b_current = next_b;
                a_current = Some(a_ptr);
            }
        }

        // Append remaining nodes from list_a
        while let Some(ptr) = a_current {
            let next = unsafe { (*ptr.as_ptr()).next };
            unsafe { (*ptr.as_ptr()).next = None };
            merged_list.add_node(ptr);
            a_current = next;
        }

        // Append remaining nodes from list_b
        while let Some(ptr) = b_current {
            let next = unsafe { (*ptr.as_ptr()).next };
            unsafe { (*ptr.as_ptr()).next = None };
            merged_list.add_node(ptr);
            b_current = next;
        }

        // Prevent original lists from dropping nodes
        list_a.end = None;
        list_a.length = 0;
        list_b.end = None;
        list_b.length = 0;

        merged_list
    }

    // Helper method to add existing nodes to the merged list
    fn add_node(&mut self, node_ptr: NonNull<Node<T>>) {
        match self.end {
            None => {
                self.start = Some(node_ptr);
                self.end = Some(node_ptr);
            }
            Some(end_ptr) => unsafe {
                (*end_ptr.as_ptr()).next = Some(node_ptr);
                self.end = Some(node_ptr);
            },
        }
        self.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        let mut list_c = LinkedList::merge(list_a, list_b);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &i in &vec_a {
            list_a.add(i);
        }
        for &i in &vec_b {
            list_b.add(i);
        }

        let mut list_c = LinkedList::merge(list_a, list_b);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as i32).unwrap());
        }
    }
}