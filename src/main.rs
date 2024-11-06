use my_data_structures::{LinkedList, Stack, Queue};

fn main() {
    println!("=== Testing Doubly Linked List ===");
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    println!("List size: {}", list.len());
    println!("Extracting elements:");
    while let Some(item) = list.pop_front() {
        println!("Got: {}", item);
    }

    println!("\n=== Testing Stack ===");
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("Stack size: {}", stack.size());
    println!("Top element: {:?}", stack.peek());
    println!("Extracting elements:");
    while let Some(item) = stack.pop() {
        println!("Got: {}", item);
    }

    println!("\n=== Testing Queue ===");
    let mut queue = Queue::new();
    queue.enqueue(100);
    queue.enqueue(200);
    queue.enqueue(300);
    println!("Queue size: {}", queue.size());
    println!("First element: {:?}", queue.peek());
    println!("Extracting elements:");
    while let Some(item) = queue.dequeue() {
        println!("Got: {}", item);
    }
}