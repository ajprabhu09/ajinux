use core::{ptr::{null, null_mut, NonNull}, fmt::Debug};

use crate::{logging::serial_print, serial_info, ksprintln};

#[repr(C)]
#[derive(Debug)]
pub struct Node<H: Sized> {
    header: H,
    next: *mut Node<H>,
    prev: *mut Node<H>,
    data: [u8; 0],
}

impl<H> Node<H> {
    pub fn from(val: *mut u8) -> *mut Node<H> {
        unsafe { core::mem::transmute::<_, *mut Node<H>>(val) }
    }
    pub const fn data_ptr<T>(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }
    pub const fn untype_ptr(&mut self) -> *mut () {
        self as *mut Node<H> as *mut ()
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
        match unsafe { self.head.as_mut() } {
            Some(data) => data.prev,
            None => 0 as *mut _,
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
        self.len += 1;
        if self.head.is_null() {
            unsafe {
                self.head = val as *mut _;
                (&mut *val).next = val as *mut _;
                (&mut *val).prev = val as *mut _;
            }
            return;
        }

        unsafe {
            let head = self.head;
            let head = &mut *head;
            let new_tail = &mut *val;
            let tail = &mut *head.prev;

            tail.next = new_tail as *mut _;
            new_tail.next = head as *mut _;
            new_tail.prev = tail as *mut _;
            head.prev = new_tail as *mut _;
        }
    }
    pub fn print_list(&self) {
        if self.len() == 0 {
            serial_info!("[]");
            return;
        }
        serial_info!("[");
        for x in self.iter() {
            serial_info!("\t {:?} {:?}", x, unsafe{&*x});
        }
        serial_info!("]");

    }
    pub fn pop_head(&mut self) -> *mut Node<H> {
        self.len -= 1;
        let Some(head) = (unsafe { self.head.as_mut() }) else {
            return null_mut();
        };

        if head.next == head.data_ptr() && head.prev == head.data_ptr() {
            head.next = null_mut();
            head.prev = null_mut();
            self.head = null_mut();
            return head.data_ptr();
        }

        let next_node = unsafe { head.next.as_mut().expect("next ptr breaks invariant") };
        let prev_node = unsafe { head.prev.as_mut().expect("prev ptr breaks invariant") };
        prev_node.next = head.next;
        next_node.prev = head.prev;
        self.head = head.next;
        return head.data_ptr();
    }

    pub fn len(&self) -> usize {
        self.len
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
        if NonNull::new(self.head).is_none() {
            return None;
        }

        if let Some(head) = NonNull::new(self.head) &&
           let Some(curr) = NonNull::new(self.curr){
            let head = head.as_ptr();
            let curr = curr.as_ptr();

            if curr == head  && !self.started {
                self.started = true;
                let next = unsafe {
                    (&*curr).next
                };
                self.curr = next;
                return Some(curr);
            }

            if curr == head {
                return None;
            }

            let next = unsafe {
                (&*curr).next
            };
            self.curr = next;
            return Some(curr);
        }
        return None;
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

#[test_case]
pub fn test_node_size() {
    assert_eq!(core::mem::size_of::<*const Node<[u8; 0]>>(), 8);
    assert_eq!(core::mem::size_of::<Node<[u8; 0]>>(), 16);
}

#[test_case]
pub fn test_linked_list() {
    use crate::info;
    serial_info!("Testing linked list");
    type H = [u8; 0];
    type Np = *mut Node<H>;

    let mut ll = LinkedList::default();

    const size: usize = 1 << 6;
    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node: Np = Node::from(other);

    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node1 = Node::from(other);

    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node2 = Node::from(other);

    ll.push_back(node);
    ll.push_back(node1);
    ll.push_back(node2);

    assert_eq!(node, ll.head);

    for l in ll.iter() {
        serial_info!("Node: {:?} {:?}", l, unsafe { &*l });
    }

    serial_info!("{:?}", data);
    assert_eq!(node2, ll.tail());
    ll.pop_head();
    ll.pop_head();
    ll.pop_head();
    ll.print_list();

    // while !ll.empty() {
    //     let val = ll.pop_head();
    //     ksprintln!("{:?}", unsafe {&*val})
    // }

    // assert_eq!(ll.len(), 0);
}
