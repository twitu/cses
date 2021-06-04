class Node:

    def __init__(self, key, value, is_red=False) -> None:
        self.key = key
        self.value = value
        self.is_red = is_red  # colour of incoming link
        self.left = None
        self.right = None

    @staticmethod
    def check_red(node):
        return node != None and node.is_red

    # self and self.right is not None
    def rotate_left(self):
        right_tree = self.right
        self.right = right_tree.left
        right_tree.left = self
        right_tree.is_red = self.is_red
        self.is_red = True
        return right_tree

    # self and self.left is not none
    def rotate_right(self):
        left_tree = self.left
        self.left = left_tree.right
        left_tree.right = self
        left_tree.is_red = self.is_red
        self.is_red = True
        return left_tree

    # self and self.right and self.left is not None
    def flip_colour(self):
        self.is_red != self.is_red
        self.left.is_red != self.left.is_red
        self.right.is_red != self.right.is_red
        return self

    def fix_up(self):

        if Node.check_red(self.right):
            self.rotate_left()

        if Node.check_red(self.left) and Node.check_red(self.left.left):
            self.rotate_right()

        if Node.check_red(self.left) and Node.check_red(self.right):
            self.flip_colour()

        return self


def get(root, key):

    if root == None:
        return None

    if key < root.key:
        return get(root.left, key)
    elif root.key < key:
        return get(root.right, key)
    else:
        return root.value


def insert(root, key, value):

    if root == None:
        return Node(key, value, True)

    if key < root.key:
        root.left = insert(root.left, key, value)
    elif root.key < key:
        root.right = insert(root.right, key, value)
    else:
        pass

    return root.fix_up()

def delete(root, key):

    pass
