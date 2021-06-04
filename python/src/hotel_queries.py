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
        return tree_index - (2**(self.n - 1) - 1) + 1 # for 1 indexed hotels

    def comparison(self, i, j):
        return self.tree[i] > self.tree[j]

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

if __name__ == "__main__":

    first_line = sys.stdin.readline().split(" ")
    n_hotels = int(first_line[0])
    m_groups = int(first_line[1])

    hotel_capacity = list(map(int, sys.stdin.readline().split(" ")))
    group_rooms = list(map(int, sys.stdin.readline().split(" ")))

    tree = SegmentTree(hotel_capacity)

    for rooms in group_rooms:
        response = tree.search(rooms)

        if response:
            (tree_index, current_rooms) = response
            tree.update_value(tree_index, current_rooms - rooms)
            print(f"{tree.original_index(tree_index)}", end=" ")
        else:
            print("0", end=" ")
