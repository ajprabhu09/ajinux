use core::ptr::{null_mut, NonNull};

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

impl<H> LinkedList<H> {
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

    pub fn pop_head(&mut self) -> *mut Node<H> {
        let head = self.head;

        if head.is_null() {
            return null_mut();
        }

        let head_next = unsafe { &*(head) }.next;
        let head_prev = unsafe { &*(head) }.prev;

        if head_prev == head {
            // only one elem
            self.head = null_mut();
        }
        unsafe {
            head_prev.as_mut().unwrap().next = head_next;
            head_next.as_mut().unwrap().prev = head_prev
        };
        unsafe {
            head.as_mut().unwrap().next = null_mut();
            head.as_mut().unwrap().prev = null_mut();
        };
        return head;
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

    for l in ll {
        serial_info!("{:?} {:?}", l, unsafe { &*l });
    }

    serial_info!("{:?}", data);
    // assert_eq!(node1, ll.tail());
}
