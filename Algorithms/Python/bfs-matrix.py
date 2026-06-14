from collections import deque

def bfs(grid):
    rows, cols = len(grid), len(grid[0])

    visit = set()
    queue = deque()
    queue.append(((0, 0), [(0, 0)]))
    visit.add((0, 0))

    length = 0
    while queue:
        for i in range(len(queue)):
            (r, c), path = queue.popleft()

            if r == rows - 1 and c == cols - 1:
                return length, path

            neighbours = [[0, 1], [0, -1], [1, 0], [-1, 0]]

            for dr, dc in neighbours:
                nr, nc = r + dr, c + dc

                if min(nr, nc) < 0 or nr == rows or nc == cols or (nr, nc) in visit or grid[nr][nc] == 1:
                    continue

                queue.append(((nr, nc), path + [(nr, nc)]))
                visit.add((nr, nc))
            
        length += 1
        
    return -1
        
grid = [[0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 1],
        [0, 1, 0, 0]]

print(bfs(grid))