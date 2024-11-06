use std::cell::RefCell;
use std::rc::Rc;
use std::os::raw::{c_int, c_void};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node {
            data,
            next: self.head.clone(),
            prev: None,
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.length -= 1;
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .expect("Something went wrong")
                .into_inner()
                .data
        })
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            Some(self.items.remove(0))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }
}

// === C API ===

#[no_mangle]
pub extern "C" fn stack_new() -> *mut Stack<c_int> {
    Box::into_raw(Box::new(Stack::new()))
}

#[no_mangle]
pub extern "C" fn stack_push(stack: *mut Stack<c_int>, value: c_int) {
    let stack = unsafe { &mut *stack };
    stack.push(value);
}

#[no_mangle]
pub extern "C" fn stack_pop(stack: *mut Stack<c_int>) -> c_int {
    let stack = unsafe { &mut *stack };
    stack.pop().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn stack_peek(stack: *const Stack<c_int>) -> c_int {
    let stack = unsafe { &*stack };
    stack.peek().copied().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn stack_size(stack: *const Stack<c_int>) -> usize {
    let stack = unsafe { &*stack };
    stack.size()
}

#[no_mangle]
pub extern "C" fn stack_free(stack: *mut Stack<c_int>) {
    unsafe {
        drop(Box::from_raw(stack));
    }
}

#[no_mangle]
pub extern "C" fn queue_new() -> *mut Queue<c_int> {
    Box::into_raw(Box::new(Queue::new()))
}

#[no_mangle]
pub extern "C" fn queue_enqueue(queue: *mut Queue<c_int>, value: c_int) {
    let queue = unsafe { &mut *queue };
    queue.enqueue(value);
}

#[no_mangle]
pub extern "C" fn queue_dequeue(queue: *mut Queue<c_int>) -> c_int {
    let queue = unsafe { &mut *queue };
    queue.dequeue().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn queue_peek(queue: *const Queue<c_int>) -> c_int {
    let queue = unsafe { &*queue };
    queue.peek().copied().unwrap_or(-1)
}

#[no_mangle]
pub extern "C" fn queue_size(queue: *const Queue<c_int>) -> usize {
    let queue = unsafe { &*queue };
    queue.size()
}

#[no_mangle]
pub extern "C" fn queue_free(queue: *mut Queue<c_int>) {
    unsafe {
        drop(Box::from_raw(queue));
    }
}

// === Тесты ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.size(), 1);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }
}