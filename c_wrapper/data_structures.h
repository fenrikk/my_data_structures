#ifndef DATA_STRUCTURES_H
#define DATA_STRUCTURES_H

#include <stddef.h>

typedef struct Stack Stack;
typedef struct Queue Queue;

// Stack functions
Stack* stack_new(void);
void stack_push(Stack* stack, int value);
int stack_pop(Stack* stack);
int stack_peek(const Stack* stack);
size_t stack_size(const Stack* stack);
void stack_free(Stack* stack);

// Queue functions
Queue* queue_new(void);
void queue_enqueue(Queue* queue, int value);
int queue_dequeue(Queue* queue);
int queue_peek(const Queue* queue);
size_t queue_size(const Queue* queue);
void queue_free(Queue* queue);

#endif // DATA_STRUCTURES_H