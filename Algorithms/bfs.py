def bfs(graph, root):
    visited = set()
    queue = []

    visited.add(root)
    queue.append(root)

    while queue:
        output = queue.pop(0)
        print(output)

        for node in graph[output]:
            if node not in visited:
                visited.add(node)
                queue.append(node)


if __name__ == "__main__":
    graph = {
    'A' : ['B','C'],
    'B' : ['D', 'E', 'F'],
    'C' : ['G'],
    'D' : [],
    'E' : [],
    'F' : ['H'],
    'G' : ['I'],
    'H' : [],
    'I' : []
    }

    bfs(graph, "A")