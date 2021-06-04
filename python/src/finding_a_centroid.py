import math
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

    def fill_size(self, tree, node_index, parent_index, node_meta_data):

        total_nodes = 0
        node_meta_data[node_index].total_nodes = 0

        for next_node in tree.edges[node_index]:

            if next_node == parent_index:
                continue

            if node_meta_data[next_node].total_nodes == -1:
                self.fill_size(tree, next_node, node_index, node_meta_data)

            total_nodes += node_meta_data[next_node].total_nodes

        node_meta_data[node_index].total_nodes = total_nodes + 1
        
    def fill_relaxed_child(self, tree, node_index, parent_index, node_meta_data):

        # preprocessing for relaxed child
        max_sub_tree_size = 0

        sorted_indexed_next_node_sized = []

        for (next_node) in tree.edges[node_index]:

            if next_node == parent_index:
                continue

            sorted_indexed_next_node_sized.append((node_meta_data[next_node].total_nodes, next_node))

        sorted_indexed_next_node_sized.sort()

        # assign relaxed child value to each relaxed child
        for relaxed_child in tree.edges[node_index]:

            parent_max_relaxed_size = 0

            # if not root node consider parent relaxed child value
            if parent_index != -1:
                parent_max_relaxed_size = node_meta_data[parent_index].relaxed_child[node_index]

            # compare with max of children size except relaxed child and skip parent
            if relaxed_child != parent_index:

                # max value cannot be given since this node is relaxed
                # assign second highest value
                if relaxed_child == sorted_indexed_next_node_sized[0][1]:
                    node_meta_data[node_index].relaxed_child[relaxed_child] = max(parent_max_relaxed_size, sorted_indexed_next_node_sized[1][0])

                # give maximum value to all other nodes
                else:
                    node_meta_data[node_index].relaxed_child[relaxed_child] = max(parent_max_relaxed_size, sorted_indexed_next_node_sized[0][0])


        if node_index == 0:
            max_sub_tree_size = sorted_indexed_next_node_sized[0][0]

            # if root has max child size less than limit then return
            if max_sub_tree_size <= math.floor(self.n / 2):
                return node_index

            # else iterate all children for same conditions
            else:
                for next_node in tree.edges[node_index]:

                    if next_node == parent_index:
                        continue

                    centroid = self.fill_relaxed_child(tree, next_node, node_index, node_meta_data)
                    
                    if centroid:
                        return centroid

        else:

            max_sub_tree_size = sorted_indexed_next_node_sized[0][0]
            max_sub_tree_size = max(max_sub_tree_size, node_meta_data[parent_index].relaxed_child[node_index])

            if max_sub_tree_size <= math.floor(self.n / 2):
                return node_index

            for next_node in tree.edges[node_index]:

                if next_node == parent_index:
                    continue

                centroid = self.fill_relaxed_child(tree, next_node, node_index, node_meta_data)

                if centroid:
                    return centroid


class MetaData:

    def __init__(self) -> None:
        self.total_nodes = -1
        self.relaxed_child = {}

    def __str__(self) -> str:
        return f"{self.total_nodes} {self.max_child_size} {self.relaxed_child}"

    def __repr__(self) -> str:
        return f"MetaData({self.total_nodes}, {self.max_child_size}, {self.relaxed_child.__repr__()})"


if __name__ == "__main__":

    n = int(sys.stdin.readline().rstrip())

    tree = Graph(n)

    for _ in range(n - 1):

        line = sys.stdin.readline().rstrip().split(" ")
        a = int(line[0]) - 1
        b = int(line[1]) - 1

        tree.add_edge(a, b)

    node_meta_data = [MetaData() for i in range(n)]
    tree.fill_size(tree, 0, 0, node_meta_data)

    centroid = tree.fill_relaxed_child(tree, 0, -1, node_meta_data) + 1
    print(centroid)
