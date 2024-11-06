#include <stdio.h>
#include "data_structures.h"

int main() {
    // Test Stack
    printf("=== Testing Stack ===\n");
    Stack* stack = stack_new();
    stack_push(stack, 10);
    stack_push(stack, 20);
    stack_push(stack, 30);
    
    printf("Stack size: %zu\n", stack_size(stack));
    printf("Top element: %d\n", stack_peek(stack));
    
    printf("Popping elements:\n");
    while (stack_size(stack) > 0) {
        printf("Got: %d\n", stack_pop(stack));
    }
    
    stack_free(stack);

    // Test Queue
    printf("\n=== Testing Queue ===\n");
    Queue* queue = queue_new();
    queue_enqueue(queue, 100);
    queue_enqueue(queue, 200);
    queue_enqueue(queue, 300);
    
    printf("Queue size: %zu\n", queue_size(queue));
    printf("Front element: %d\n", queue_peek(queue));
    
    printf("Dequeuing elements:\n");
    while (queue_size(queue) > 0) {
        printf("Got: %d\n", queue_dequeue(queue));
    }
    
    queue_free(queue);
    
    return 0;
}