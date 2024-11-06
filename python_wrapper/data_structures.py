from ctypes import *
import os

lib_path = os.path.join(os.path.dirname(__file__), "..", "target", "release", "libmy_data_structures.so")
_lib = CDLL(lib_path)

class Stack:
    def __init__(self):
        self._stack = _lib.stack_new()
    
    def push(self, value: int):
        _lib.stack_push(self._stack, value)
    
    def pop(self) -> int:
        result = _lib.stack_pop(self._stack)
        if result == -1:
            raise IndexError("Stack is empty")
        return result
    
    def peek(self) -> int:
        result = _lib.stack_peek(self._stack)
        if result == -1:
            raise IndexError("Stack is empty")
        return result
    
    def size(self) -> int:
        return _lib.stack_size(self._stack)
    
    def __del__(self):
        if hasattr(self, '_stack'):
            _lib.stack_free(self._stack)

class Queue:
    def __init__(self):
        self._queue = _lib.queue_new()
    
    def enqueue(self, value: int):
        _lib.queue_enqueue(self._queue, value)
    
    def dequeue(self) -> int:
        result = _lib.queue_dequeue(self._queue)
        if result == -1:
            raise IndexError("Queue is empty")
        return result
    
    def peek(self) -> int:
        result = _lib.queue_peek(self._queue)
        if result == -1:
            raise IndexError("Queue is empty")
        return result
    
    def size(self) -> int:
        return _lib.queue_size(self._queue)
    
    def __del__(self):
        if hasattr(self, '_queue'):
            _lib.queue_free(self._queue)