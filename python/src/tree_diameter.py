import sys

# Model tree as an undirected acyclic graph
class Graph:

    def __init__(self, n) -> None:
        
        self.n = n
        self.edges = [[] for i in range(n)]

    def add_edge(self, a, b):
        self.edges[a].append(b)
        self.edges[b].append(a)


    def bfs(self, start):

        queue = [(start, 0)]
        index = 0
        cur_node = None
        visited = [ False for i in range(self.n)]

        while index != len(queue):

            cur_node = queue[index]
            (cur_node_id, step_count) = cur_node

            index += 1
            visited[cur_node_id] = True

            for next_node in self.edges[cur_node_id]:

                if not visited[next_node]:
                    queue.append((next_node, step_count + 1))

        return cur_node


if __name__ == "__main__":

    n = int(sys.stdin.readline().rstrip())

    tree = Graph(n)

    for _ in range(n - 1):

        line = sys.stdin.readline().rstrip().split(" ")
        a = int(line[0]) - 1
        b = int(line[1]) - 1

        tree.add_edge(a, b)

    diameter_start = tree.bfs(0)
    diameter_end = tree.bfs(diameter_start[0])

    print(diameter_end[1])
