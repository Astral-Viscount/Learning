def dfs(grid, current_r, current_c, visit, current_path, all_path):
    rows, cols = len(grid), len(grid[0])

    if (min(current_r, current_c) < 0 or current_r == rows or current_c == cols or (current_r, current_c) in visit or grid[current_r][current_c] == 1):
        return 0

    visit.add((current_r, current_c))
    current_path.append((current_r, current_c))

    if current_r == rows - 1 and current_c == cols - 1:
        all_path.append(list(current_path))
        count = 1
    else:
        count = 0

        count += dfs(grid, current_r + 1, current_c, visit, current_path, all_path)
        count += dfs(grid, current_r - 1, current_c, visit, current_path, all_path)
        count += dfs(grid, current_r, current_c + 1, visit, current_path, all_path)
        count += dfs(grid, current_r, current_c - 1, visit, current_path, all_path)

    current_path.pop()
    visit.remove((current_r, current_c))

    return count

grid = [[0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 1],
        [0, 1, 0, 0]]

paths = []
        
dfs(grid, 0, 0, set(), [], paths)

print(f"Total paths: {len(paths)}")
for idx, path in enumerate(paths, 1):
    print(f"Path {idx}: {path}")
