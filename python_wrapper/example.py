from data_structures import Stack, Queue

def test_stack():
    print("=== Testing Stack ===")
    stack = Stack()
    stack.push(10)
    stack.push(20)
    stack.push(30)
    
    print(f"Stack size: {stack.size()}")
    print(f"Top element: {stack.peek()}")
    
    print("Popping elements:")
    while stack.size() > 0:
        print(f"Got: {stack.pop()}")

def test_queue():
    print("\n=== Testing Queue ===")
    queue = Queue()
    queue.enqueue(100)
    queue.enqueue(200)
    queue.enqueue(300)
    
    print(f"Queue size: {queue.size()}")
    print(f"Front element: {queue.peek()}")
    
    print("Dequeuing elements:")
    while queue.size() > 0:
        print(f"Got: {queue.dequeue()}")

if __name__ == "__main__":
    test_stack()
    test_queue()