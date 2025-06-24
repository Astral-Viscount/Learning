queue = []

# Enqueue - Adds a new element to the queue.
queue.append('A')
queue.append('B')
queue.append('C')
print("Queue: ", queue)

# Peek - Returns the first element in the queue. 
frontElement = queue[0]
print("Peek: ", frontElement)

# Dequeue - Removes and returns the first (front) element from the queue.
poppedElement = queue.pop(0)
print("Dequeue: ", poppedElement)

print("Queue after Dequeue: ", queue)

# isEmpty - Returns the first element in the queue.
isEmpty = not bool(queue)
print("isEmpty: ", isEmpty)

# Size - Finds the number of elements in the queue.
print("Size: ", len(queue))