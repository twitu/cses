import sys

class Graph:

    def __init__(self, n) -> None:
        
        self.n = n
        self.edges = [[] for i in range(n)]

    def add_edge(self, start, end):
        self.edges[start].append(end)
    
    def check_connectivity(self):

        def dfs(node, visited, chain_length):
            visited[node] = True
            chain_length += 1

            if chain_length == self.n:
                return chain_length

            for edge in self.edges[node]:

                if not visited[edge]:
                    chain_length = dfs(edge, visited, chain_length)

                if chain_length == self.n:
                    return chain_length

            chain_length -= 1
            visited[node] = False

        visited = [False for i in range(self.n)]

        max_chain_length = dfs(0, visited, 0)

        return visited, max_chain_length


if __name__ == "__main__":

    first_line = sys.stdin.readline().rstrip().split(" ")
    n_cities = int(first_line[0])
    m_flights = int(first_line[1])

    g = Graph(n_cities)

    for _ in range(m_flights):

        line = sys.stdin.readline().rstrip().split(" ")
        start = int(line[0]) - 1
        end = int(line[1]) - 1
        g.add_edge(start, end)


    visited, max_chain_length = g.check_connectivity()
    print(visited)
    print(max_chain_length)
