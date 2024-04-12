/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        let mut current = self.start;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(unsafe { &(*node.as_ptr()).val });
            }

            current = unsafe { (*node.as_ptr()).next };
            current_index += 1;
        }

        None
    }

    pub fn reverse(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = &mut *node_ptr.as_ptr();
                let next_ptr = node.next;
                node.next = node.prev;
                node.prev = next_ptr;
                current = next_ptr;
            }
        }
        std::mem::swap(&mut self.start, &mut self.end);
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;

        while let Some(node) = current {
            write!(f, "{}", unsafe { &(*node.as_ptr()).val })?;

            if unsafe { (*node.as_ptr()).next }.is_some() {
                write!(f, ", ")?;
            }

            current = unsafe { (*node.as_ptr()).next };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for &val in &original_vec {
            list.add(val);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for &val in &original_vec {
            list.add(val);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}