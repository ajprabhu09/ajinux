use core::{
    fmt::Debug,
    ptr::{null_mut},
};

use crate::{serial_error, serial_info, utils::ptr_utils::{as_ref_mut}};
extern crate static_assertions as sa;
#[repr(C)]
// #[derive(Debug)]
pub struct Node<H: Sized> {
    pub header: H,
    pub next: *mut Node<H>,
    pub prev: *mut Node<H>,
}

impl<H> Node<H> {
    pub fn data_ptr_skip_header(&mut self) -> *mut () {
        return core::ptr::addr_of_mut!(self.next) as *mut ();
    }
    pub fn from(val: *mut u8) -> *mut Node<H> {
        val as *mut Node<H>
    }
    pub const fn untype_ptr_mut(&mut self) -> *mut Node<H>{
        self as *mut Node<H>
    }
    pub const fn untype_ptr(&self) -> *const Node<H>{
        self as *const Node<H>
    }
}

#[repr(C)]
pub struct LinkedList<H> {
    len: usize,
    head: *mut Node<H>,
}

impl<H: Debug> LinkedList<H> {
    pub fn empty(&self) -> bool {
        return self.head.is_null();
    }

    pub fn tail(&mut self) -> *mut Node<H> {
        match as_ref_mut(self.head) {
            Some(data) => data.prev,
            None => null_mut(),
        }
    }
    pub const fn default() -> Self {
        LinkedList {
            head: 0 as *mut _,
            len: 0,
        }
    }
    pub fn push_front(&mut self, val: *mut Node<H>) {
        self.push_back(val);
        self.head = val as *mut _;
    }

    pub fn push_back(&mut self, val: *mut Node<H>) {
        let Some(val_ref) = as_ref_mut(val) else {
            panic!("cannot add null to linked list");
        };

        self.len += 1;

        let Some(head) = as_ref_mut(self.head) else {
            self.head = val;
            let head = as_ref_mut(self.head).unwrap();
            head.next = val;
            head.prev = val;
            return;
        };
        if head.next == self.head && head.prev == self.head {
            head.next = val;
            head.prev = val;
            val_ref.next = self.head;
            val_ref.prev = self.head;
            return;
        }

        let last = as_ref_mut(head.prev).expect("breaks invariant of circular list");

        last.next = val;
        val_ref.prev = last as *mut _;
        val_ref.next = head as *mut _;

        head.prev = val;

        return;
    }
    // pub fn print_list(&self) {
    //     if self.len() == 0 {
    //         serial_info!("[]");
    //         return;
    //     }
    //     serial_info!("[");
    //     for x in self.iter() {
    //         serial_info!("\t {:?} {:?}", x, unsafe { &*x });
    //     }
    //     serial_info!("]");
    // }
    pub fn pop_head(&mut self) -> *mut Node<H> {
        let head_ptr = self.head;
        let Some(head) = as_ref_mut(self.head) else {
            return null_mut();
        };
        assert!(self.len != 0);

        self.len -= 1;

        if head.next == head_ptr && head.prev == head_ptr {
            head.next = null_mut();
            head.prev = null_mut();
            self.head = null_mut();
            return head_ptr;
        }

        let next_node = as_ref_mut(head.next).expect("next ptr breaks invariant");
        let prev_node = as_ref_mut(head.prev).expect("prev ptr breaks invariant");
        prev_node.next = head.next;
        next_node.prev = head.prev;
        self.head = head.next;
        return head_ptr;
    }

    pub fn remove(&mut self, node: *mut Node<H>) -> bool {
        // assert node in list
        let Some(node) = self.iter().find(|x| *x == node) else {
            return false;
        };

        let node_ref = as_ref_mut(node).unwrap();

        let head_ptr = self.head;
        let Some(head) = as_ref_mut(head_ptr) else {
            serial_error!("head is empty");
            return false;
        };

        assert!(self.len != 0);
        self.len -= 1;
        if head.next == head_ptr && head.prev == head_ptr {
            head.next = null_mut();
            head.prev = null_mut();
            self.head = null_mut();
            return true;
        }

        let next_node = unsafe { as_ref_mut(node_ref.next).expect("next ptr breaks invariant") };
        let prev_node = unsafe { as_ref_mut(node_ref.prev).expect("prev ptr breaks invariant") };
        prev_node.next = node_ref.next;
        next_node.prev = node_ref.prev;
        if self.head == node {
            self.head = next_node;
        }

        return true;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn print_ptrlist(&self, max: usize) {
        serial_info!("PTRS - [");
        let mut max = max;
        for (_idx, x) in self.iter().enumerate() {
            let ref_val = unsafe { &*x };
            if max == 0 {
                serial_info!(".... [truncated]");
                break;
            }
            max -= 1;
            serial_info!("{:?} next-{:?} prev-{:?} ", x, ref_val.next, ref_val.prev);
        }
        serial_info!("]");
    }

    pub fn iter(&self) -> LLIter<H> {
        LLIter {
            head: self.head,
            curr: self.head,
            started: false,
        }
    }
}

pub struct LLIter<H> {
    head: *mut Node<H>,
    curr: *mut Node<H>,
    started: bool,
}

impl<H> Iterator for LLIter<H> {
    type Item = *mut Node<H>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            return None;
        }

        if self.curr == self.head && self.started {
            return None;
        }
        // serial_info!("finish {:?} {:?} ", self.curr, self.head);

        self.started = true;

        let ret_val = self.curr;
        self.curr = (unsafe { self.curr.as_ref() })?.next;

        return Some(ret_val);
    }
}

impl<H> IntoIterator for LinkedList<H> {
    type Item = *mut Node<H>;

    type IntoIter = LLIter<H>;

    fn into_iter(self) -> Self::IntoIter {
        LLIter {
            head: self.head,
            curr: self.head,
            started: false,
        }
    }
}
#[cfg(test)]
mod tests {
    use core::ptr::{null_mut};

    use crate::{
        datastructures::no_alloc::linked_list::{LinkedList, Node},
        serial_info,
    };

    #[test_case]
    pub fn test_node_size() {
        assert_eq!(core::mem::size_of::<*const Node<[u8; 0]>>(), 8);
        assert_eq!(core::mem::size_of::<Node<[u8; 0]>>(), 16);
    }

    #[test_case]
    pub fn test_linked_list() {
        
        serial_info!("Testing linked list");
        type H = [u8; 0];
        type Np = *mut Node<H>;

        let mut ll = LinkedList::default();

        const size: usize = 1 << 6;
        let mut data: [u8; size] = [0; size];
        let other = data.as_mut_ptr();
        let node: Np = Node::from(other);

        let mut data: [u8; size] = [0; size];
        let other = data.as_mut_ptr();
        let node1 = Node::from(other);

        let mut data: [u8; size] = [0; size];
        let other = data.as_mut_ptr();
        let node2 = Node::from(other);

        ll.push_back(node);
        ll.push_back(node1);
        ll.push_back(node2);

        assert_eq!(node, ll.head);
        assert_eq!(ll.len(), 3);
        ll.print_ptrlist(usize::MIN);

        for l in ll.iter().zip([node, node1, node2]) {
            assert_eq!(l.0 as *mut u8, l.1 as *mut u8);
            // serial_info!("Node: {:?} {:?}", l, unsafe { &*l.0 });
        }

        serial_info!("{:?}", data);
        assert_eq!(node2, ll.tail());
        ll.pop_head();
        ll.pop_head();
        ll.pop_head();

        assert_eq!(ll.len(), 0);

        assert_eq!(ll.head, null_mut());

        // ll.print_list();

        ll.push_back(node);
        ll.push_back(node1);
        ll.push_back(node2);

        assert_eq!(ll.head, node);
        assert_eq!(ll.len(), 3);

        ll.remove(node1);
        assert_eq!(ll.len(), 2);
        ll.print_ptrlist(usize::MAX);
        serial_info!("Middle removed");
        for l in ll.iter().zip([node, node2]) {
            assert_eq!(l.0 as *mut u8, l.1 as *mut u8);
            // serial_info!("Node: {:?} {:?}", l, unsafe { &*l.0 });
        }

        serial_info!("not preseent removed");

        ll.remove(node1);

        assert_eq!(ll.len(), 2);

        for l in ll.iter().zip([node, node2]) {
            assert_eq!(l.0 as *mut u8, l.1 as *mut u8);
            // serial_info!("Node: {:?} {:?}", l, unsafe { &*l.0 });
        }

        serial_info!("remaining removed");

        ll.remove(node2);
        assert_eq!(ll.len(), 1);

        for l in ll.iter().zip([node]) {
            assert_eq!(l.0 as *mut u8, l.1 as *mut u8);
            // serial_info!("Node: {:?} {:?}", l, unsafe { &*l.0 });
        }

        serial_info!("final removed");

        ll.remove(node);
        assert_eq!(ll.len(), 0);

        serial_info!("final removed");

        assert_eq!(ll.head, null_mut());

        // ll.print_list();

        // while !ll.empty() {
        //     let val = ll.pop_head();
        //     ksprintln!("{:?}", unsafe {&*val})
        // }

        // assert_eq!(ll.len(), 0);
    }
}
