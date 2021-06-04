import sys
import math

class SegmentTree:

    @staticmethod
    def parent(i):
        return (i - 1) // 2

    @staticmethod
    def left(i):
        return i * 2 + 1

    @staticmethod
    def right(i):
        return i * 2 + 2

    @staticmethod
    def sibling(i):
        if i % 2 == 0:
            return i - 1
        else:
            return i + 1

    # should only be called using an index from the last level
    def original_index(self, tree_index):
        return tree_index - (2**(self.n - 1) - 1)

    # should only be called using an index within bounds of the original array
    def tree_index(self, original_index):
        return original_index + (2**(self.n - 1) - 1)

    def comparison(self, i, j):
        return self.tree[i] < self.tree[j]

    def __init__(self, values) -> None:
        self.n = math.ceil(math.log2(len(values))) + 1
        self.tree = [0 for i in range(2**self.n - 1)]

        # fill initial values
        for (i, value) in zip(range(2**(self.n - 1) - 1, 2**self.n - 1), values):
            self.tree[i] = value

        for i in range(2**self.n - 1 - 1, 0, -2):
            if self.comparison(i, i - 1):
                self.tree[self.parent(i)] = self.tree[i]
            else:
                self.tree[self.parent(i)] = self.tree[i - 1]

    def search(self, value):
        index = 0

        if value > self.tree[index]:
            return None

        while index < (2**(self.n - 1) - 1):

            if value <= self.tree[self.left(index)]:
                index = self.left(index)
                continue

            if value <= self.tree[self.right(index)]:
                index = self.right(index)
                continue

        return (index, self.tree[index])

    def update_value(self, tree_index, value):
        self.tree[tree_index] = value

        while tree_index != 0:
            sibling_index = self.sibling(tree_index)

            if self.comparison(tree_index, sibling_index):
                self.tree[self.parent(tree_index)] = self.tree[tree_index]
            else:
                self.tree[self.parent(tree_index)] = self.tree[sibling_index]

            tree_index = self.parent(tree_index)

    def return_original_values(self):
        return self.tree[2**(self.n - 1) - 1, 2**self.n - 1]

    def min_range_query(self, a, b, left_boundary, right_boundary, index):
        mid_way = left_boundary + (right_boundary - left_boundary) // 2

        if a <= left_boundary and right_boundary <= b:
            return self.tree[index]

        elif right_boundary < a or b < left_boundary:
            return sys.maxsize

        else:
            return min(
                self.min_range_query(a, b, left_boundary, mid_way, self.left(index)),
                self.min_range_query(a, b, mid_way + 1, right_boundary, self.right(index))
            )




if __name__ == "__main__":

    first_line = sys.stdin.readline().split(" ")
    n_numbers = int(first_line[0])
    q_queries = int(first_line[1])

    values = list(map(int, sys.stdin.readline().split(" ")))
    tree = SegmentTree(values)

    for q in range(q_queries):

        query = list(map(int, sys.stdin.readline().split(" ")))

        # update query
        if query[0] == 1:
            index = query[1] - 1
            value = query[2]
            tree.update_value(tree.tree_index(index), value)

        # look up query
        else:
            a = query[1] - 1
            b = query[2] - 1
            print(f"{tree.min_range_query(a, b, 0, n_numbers - 1, 0)}")
