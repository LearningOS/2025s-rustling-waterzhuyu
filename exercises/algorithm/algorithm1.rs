/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>, // NonNull<T> wraps a raw pointer of T, i.e. *mut T.
}                                   // and promise it's non-null.
        // using NonNull (raw pointer) manage memory, but it doesn't own memory by itself

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
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
        // The actual ownership is moved by Box
        let mut node = Box::new(Node::new(obj)); // with ownership
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

    // ---------------------------------------------------------------------------------------
    // Recursive Implementation
    // pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	// {
	// 	//TODO
    //     let merged = Self::merge_nodes(list_a.start, list_b.start);
	// 	Self {
    //         length: list_a.length + list_b.length,
    //         start: merged,
    //         end: LinkedList::node_after_ith_node(merged,  (list_a.length + list_b.length) as i32),
    //     }
	// }

    // fn node_after_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
    //     match node {
    //         None => None,
    //         Some(next_ptr) => match index {
    //             0 => Some(unsafe {
    //                 NonNull::new_unchecked(next_ptr.as_ptr())
    //             }),
    //             _ => LinkedList::node_after_ith_node(unsafe {(*next_ptr.as_ptr()).next}, index - 1),
    //         }
    //     }
    // }

    // fn merge_nodes(node_a: Option<NonNull<Node<T>>>, node_b: Option<NonNull<Node<T>>>) -> Option<NonNull<Node<T>>> {
    //     match (node_a, node_b) {
    //         (None, None) => None,
    //         (Some(ptr), None) | (None, Some(ptr)) => Some(ptr),
    //         (Some(l), Some(r)) => {
    //             let val_a = unsafe { &(*l.as_ptr()).val };
    //             let val_b = unsafe { &(*r.as_ptr()).val };
    //             if val_a < val_b {
    //                 unsafe {
    //                     (*l.as_ptr()).next = Self::merge_nodes((*l.as_ptr()).next, Some(r));
    //                 }
    //                 Some(l)
    //             } else {
    //                 unsafe {
    //                     (*r.as_ptr()).next = Self::merge_nodes(Some(l), (*r.as_ptr()).next);
    //                 }
    //                 Some(r)
    //             }
    //         }
    //     }
    // }
    // ----------------------------------------------------------------------------------------

    //TODO: Shall we take ownership of Node of list_a & list_b?
    // Version1: don't take ownership
    // V2: takes ownership
    // pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self 
    // where T: PartialOrd + Clone   
    // {
    // // Iterative implementation
    //     let mut merged = LinkedList::new();
    //     let mut ptr_a = list_a.start;  // move ownership
    //     let mut ptr_b = list_b.start;  // move ownership

    //     while ptr_a.is_some() && ptr_b.is_some() {
    //         let val_a = unsafe { (*ptr_a.unwrap().as_ptr()).val.clone() };
    //         let val_b = unsafe { (*ptr_b.unwrap().as_ptr()).val.clone() };

    //         if val_a < val_b {
    //             merged.add(val_a);
    //             ptr_a = unsafe {
    //                 (*ptr_a.unwrap().as_ptr()).next
    //             }
    //         } else {
    //             merged.add(val_b);
    //             ptr_b = unsafe {
    //                 (*ptr_b.unwrap().as_ptr()).next
    //             }
    //         }
    //     }

    //     while ptr_a.is_some() {
    //         let val_a = unsafe { (*ptr_a.unwrap().as_ptr()).val.clone() };
    //         merged.add(val_a);
    //         ptr_a = unsafe { (*ptr_a.unwrap().as_ptr()).next };
    //     }

    //     while ptr_b.is_some() {
    //         let val_b = unsafe { (*ptr_b.unwrap().as_ptr()).val.clone() };
    //         merged.add(val_b);
    //         ptr_b = unsafe { (*ptr_b.unwrap().as_ptr()).next };
    //     }

    //     merged.end = match merged.start {
    //         Some(mut node) => {
    //             while unsafe { node.as_ref().next.is_some() } {
    //                 node = unsafe { node.as_ref().next.unwrap() };
    //             }
    //             Some(node)
    //         },
    //         None => None
    //     };

    //     merged
    // }
    
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self 
    where T: PartialOrd
    {
        let mut merged = LinkedList::new();
        let mut ptr_a = list_a.start;  // move ownership
        let mut ptr_b = list_b.start;  // move ownership

        let mut curr = &mut merged.start;
        while ptr_a.is_some() && ptr_b.is_some() {
            unsafe {
                if (*ptr_a.unwrap().as_ptr()).val < (*ptr_b.unwrap().as_ptr()).val {
                    *curr = ptr_a;
                    ptr_a = (*ptr_a.unwrap().as_ptr()).next;
                } else {
                    *curr = ptr_b;
                    ptr_b = (*ptr_b.unwrap().as_ptr()).next;
                }

                curr = &mut (*curr.unwrap().as_ptr()).next;
            }
        }

        *curr = if ptr_a.is_some() {
            ptr_a
        } else {
            ptr_b
        };

        merged.length = list_a.length + list_b.length;
        merged.end = match merged.start {
            Some(mut node) => {
                while unsafe { node.as_ref().next.is_some() } {
                    node = unsafe { (*node.as_ptr()).next.unwrap() };
                }
                Some(node)
            },
            None => None
        };

        merged
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
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}

    // #[test]
    // fn test_ownership_management() {

    // }
}