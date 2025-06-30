def dfs(graph, root):
    visited = set()
    stack = []

    visited.add(root)
    stack.append(root)

    while stack:
        output = stack.pop()
        print(output)

        for node in reversed(graph[output]):
            if node not in visited:
                visited.add(node)
                stack.append(node)


if __name__ == "__main__":
    graph = {
    'A' : ['B','G'],
    'B' : ['C', 'D', 'E'],
    'C' : [],
    'D' : [],
    'E' : ['F'],
    'F' : [],
    'G' : ['H'],
    'H' : ['I'],
    'I' : [],
    }

    dfs(graph, "A")