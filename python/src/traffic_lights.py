import sys


class Tree:

    def __init__(self, value, left, right) -> None:
        self.value = value
        self.left = left
        self.right = right

    def insert(self, value):

        if value <= self.value:
            self.left = (
                self.left.insert(value)
                if self.left != None
                else Tree(value, None, None)
            )
        else:
            self.right = (
                self.right.insert(value)
                if self.right != None
                else Tree(value, None, None)
            )

        return self

    def search(self, value):

        if value < self.value:
            return self.left.search(value) if self.left != None else False
        elif self.value < value:
            return self.right.search(value) if self.right != None else False
        else:
            return True

    def delete(self, value):

        if value < self.value:
            self.left = self.left.delete(value) if self.left != None else None
            return self
        elif self.value < value:
            self.right = self.right.delete(value) if self.right != None else None
            return self

        # found value
        else:

            # leaf node
            if self.left == None and self.right == None:
                return None

            # one child (right case)
            elif self.left == None and self.right != None:
                return self.right

            # one child (left case)
            elif self.left != None and self.right == None:
                return self.left

            # both child case
            else:

                inorder_successor = self.right.remove_inorder_successor()
                self.value = inorder_successor.value
                return self

    def find_range(self, value, cur_range):

        if self == None:
            return cur_range

        if value < self.value:
            (left_boundary, _) = cur_range
            return (
                self.left.find_range(value, (left_boundary, self.value))
                if self.left != None
                else cur_range
            )

        elif self.value < value:
            (_, right_boundary) = cur_range
            return (
                self.right.find_range(value, (self.value, right_boundary))
                if self.right != None
                else cur_range
            )

        else:
            return cur_range


    def find_max(self):

        if self == None:
            return None

        if self.right == None:
            return self.value
        else:
            return self.right.find_max()

    def remove_inorder_successor(self):

        if self == None:
            raise Exception

        # go left if possible
        if self.left != None:
            (new_left, successor) = self.left.inorder_successor()
            self.left = new_left
            return (self, successor)

        # if no left branch go right
        elif self.right != None:
            (new_right, successor) = self.left.inorder_successor()
            self.right = new_right
            return (self, successor)

        # no other child then return value
        else:
            return (None, self)


if __name__ == "__main__":

    first_line = sys.stdin.readline().rstrip().split(" ")
    street_length = int(first_line[0])
    num_street_light = int(first_line[1])

    second_line = map(int, sys.stdin.readline().split(" "))

    # tree with street lengths
    unlit_range = Tree(street_length, None, None)

    # emptry tree with no lights
    street_lights = None

    for new_light in second_line:

        if street_lights == None:
            street_lights = Tree(new_light, None, None)
        else:
            street_lights = street_lights.insert(new_light)

        (left_boundary, right_boundary) = street_lights.find_range(
            new_light, (0, street_length)
        )
        prev_unlit = right_boundary - left_boundary
        new_left_unlit = new_light - left_boundary
        new_right_unlit = right_boundary - new_light

        unlit_range: Tree = unlit_range.delete(prev_unlit)

        # in case tree is empty
        if unlit_range == None:
            unlit_range = Tree(new_left_unlit, None, None)
        else:
            unlit_range = unlit_range.insert(new_left_unlit)

        unlit_range = unlit_range.insert(new_right_unlit)

        # largest unlit section
        print(f"{unlit_range.find_max()}", end=" ")
