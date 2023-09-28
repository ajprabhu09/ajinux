use core::{marker::PhantomData, ptr::NonNull};


#[repr(C)]
#[derive(Debug)]
pub struct Node {
    next: *mut Node,
    prev: *mut Node,
    data: [u8; 0]
}

impl Node {
    pub fn from(val: *mut u8) ->*mut Node {
        unsafe { core::mem::transmute::<_,*mut Node>(val) }
    }
    pub const fn data_ptr<T>(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }
    pub const fn untype_ptr(&mut self) -> *mut () {
        self as *mut Node as *mut ()
    }
}

#[repr(C)]
pub struct LinkedList {
    head: *mut Node,
}

impl LinkedList {
    pub fn tail(&mut self) -> *mut Node {
        
        match unsafe { self.head.as_mut() } {
            Some(data) =>  data.prev,
            None => 0 as *mut _,
        }
    
    }
    pub const fn default() -> Self {
        LinkedList { head: 0 as *mut _ }
    }
    pub fn push_front(&mut self, val: *mut Node) {
        unsafe {
            self.push_back(val);
        }
        self.head = val as *mut _;
    }

    pub fn push_back(&mut self, val: *mut Node) {
        unsafe {
            match NonNull::new(self.head) {
                Some(head) => {
                    let head = head.as_ptr();
                    
                    let head = &mut *head;
                    let new_tail = &mut *val;
                    let tail = &mut *head.prev;

                    tail.next = new_tail as *mut _;
                    new_tail.next = head as *mut _;
                    new_tail.prev = tail as *mut _;
                    head.prev = new_tail as *mut _;
                }
                None => {
                    self.head =val as *mut _;
                    (&mut *val).next = val as *mut _;
                    (&mut *val).prev = val as *mut _;
                }
            }
        }
    }
    pub fn iter(&self) -> LLIter {
        LLIter { head: self.head, curr: self.head, started: false }
    }
}

pub struct LLIter {
    head: *mut Node,
    curr: *mut Node,
    started: bool,
}




impl Iterator for LLIter {
    type Item = *mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        if NonNull::new(self.head).is_none() {
            return None
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


impl IntoIterator for LinkedList {
    type Item = *mut Node;

    type IntoIter = LLIter;

    fn into_iter(self) -> Self::IntoIter {
        LLIter{
            head: self.head,
            curr: self.head,
            started: false,
        }
    }

}

#[test_case]
pub fn test_node_size() {

    assert_eq!(core::mem::size_of::<*const Node>(), 8);
    assert_eq!(core::mem::size_of::<Node>(), 16);
}


#[test_case]
pub fn test_linked_list() {
    use crate::info;
    info!("Testing linked list");

    let mut ll = LinkedList::default();

    const size: usize = 1 << 6;
    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node =  Node::from(other);

    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node1 =  Node::from(other);

    let mut data: [u8; size] = [0; size];
    let other = data.as_mut_ptr();
    let mut node2 =  Node::from(other);

    ll.push_back(node);
    ll.push_back(node1);
    ll.push_back(node2);


    assert_eq!(node, ll.head);

    for l in ll {
        info!("{:?} {:?}", l,unsafe{&*l});
    }

    info!("{:?}", data);

    // assert_eq!(node1, ll.tail());

}