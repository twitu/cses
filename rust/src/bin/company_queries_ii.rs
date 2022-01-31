use std::{
    io::{stdin, Read},
    iter,
    ops::RangeInclusive,
    rc::Rc,
};

#[derive(Debug)]
struct FunctionalGraph {
    size: usize,
    // for a functional graph each node has only one outward edge
    next_node: Vec<usize>,
    // inverted view of functional graph is a tree with multiple
    // root nodes
    tree_view: Rc<Vec<Vec<usize>>>,
    // find and store 1st, 2nd, 4th ... ancestors and so on
    // usize::MAX indicates no ancestor
    ancestors: Vec<Vec<usize>>,
    // max ancestors to store for each node
    ancestor_limit: usize,
    // euler traversal start stop value
    euler_traversal: Vec<RangeInclusive<usize>>,
    // depth information about each node
    depth: Vec<usize>,
}

impl FunctionalGraph {
    fn new(next_node: Vec<usize>) -> Self {
        let size = next_node.len();
        let mut tree_view = vec![vec![]; size];

        let ancestor_limit: usize = f64::ceil(f64::log2(size as f64)).round() as usize;
        let ancestors = vec![vec![usize::MAX; ancestor_limit]; size];
        let euler_traversal = vec![0..=0; size];
        let heights = vec![0; size];

        // create tree view by taking
        // reverse links of given nodes
        for (from, &to) in next_node.iter().enumerate() {
            // skip for dummy next value
            if to == usize::MAX {
                continue;
            }

            tree_view[to].push(from);
        }

        let tree_view = Rc::new(tree_view);

        let mut graph = FunctionalGraph {
            size,
            next_node,
            tree_view,
            ancestors,
            ancestor_limit,
            euler_traversal,
            depth: heights,
        };

        // start ancestor analysis from root node which is given as 0
        graph.ancestor_analysis(0, usize::MAX, 0, 0);
        return graph;
    }

    // performs a dfs from the root and fill ancestor jump table
    // also do a euler traversal and mark euler ranges to perform
    // ancestry check in O(1)
    fn ancestor_analysis(
        &mut self,
        node: usize,
        parent: usize,
        mut euler_count: usize,
        depth: usize,
    ) -> usize {
        // populate ancestor table if it is not a root node
        // a root node has usize::MAX for parent node
        if parent != usize::MAX {
            self.ancestors[node][0] = parent;

            for i in 1..self.ancestor_limit {
                self.ancestors[node][i] = self.ancestors[self.ancestors[node][i - 1]][i - 1];

                if self.ancestors[node][i] == usize::MAX {
                    break;
                }
            }
        }

        self.depth[node] = depth;
        let euler_start = euler_count;

        let children = self.tree_view.clone();
        for &child in &children[node] {
            euler_count = self.ancestor_analysis(child, node, euler_count + 1, depth + 1);
        }

        self.euler_traversal[node] = euler_start..=euler_count;

        euler_count + 1
    }

    // find lower common ancestor between a and b
    // start from the employee with lower depth
    // and check if the other employee's (higher depth) euler traversal is
    // contained within it or it's ancestors traversal
    // use ancestor jump table to ancestors in log time
    fn lca(&self, emp_a: usize, emp_b: usize) -> usize {
        let (deep_emp, check_emp) = if self.depth[emp_a] < self.depth[emp_b] {
            (emp_b, emp_a)
        } else {
            (emp_a, emp_b)
        };

        self.find_ancestor(deep_emp, check_emp)
    }

    // find common ancestor between employee deeper in the tree
    // and check employee. Using check employee's euler traversal
    // for doing the comparisons
    fn find_ancestor(&self, deep_emp: usize, check_emp: usize) -> usize {
        // is directly an ancestor
        if self.check_euler_traversal_container(deep_emp, check_emp) {
            return check_emp;
        }
        // check for common ancestor
        else {
            if let Some((index, _)) = self.ancestors[check_emp]
                .iter()
                .enumerate()
                .filter(|(_, ancestor)| **ancestor != usize::MAX) // remove default values
                .rev()
                // find highest ancestor that is not common
                .filter(|(_, &ancestor)| !self.check_euler_traversal_container(deep_emp, ancestor))
                .take(1)
                .next()
            {
                // found lower uncommon ancestor continue search from there
                return self.find_ancestor(deep_emp, self.ancestors[check_emp][index]);
            } else {
                // all ancestors are common but check emp is itself not an ancestor
                // immediate parent must be lca
                return self.ancestors[check_emp][0];
            }
        }
    }

    // check if check employee is a parent of deeper employee
    // by comparing their euler traversal ranges
    fn check_euler_traversal_container(&self, deep_emp: usize, check_emp: usize) -> bool {
        let start = self.euler_traversal[deep_emp].start();
        let end = self.euler_traversal[deep_emp].end();

        self.euler_traversal[check_emp].contains(start)
            && self.euler_traversal[check_emp].contains(end)
    }
}

// https://cses.fi/problemset/task/1688
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let first_line: Vec<usize> = input
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|val| val.parse().unwrap())
        .collect();
    let _n_employees: usize = first_line[0];
    let q_queries: usize = first_line[1];

    let parent_list: Vec<usize> = iter::once(usize::MAX) // no parent for first node
        .chain(
            input
                .next()
                .unwrap()
                .split(' ')
                .map(|value| value.parse::<usize>().unwrap() - 1),
        )
        .collect();

    let graph = FunctionalGraph::new(parent_list);

    for _ in 0..q_queries {
        let query: Vec<usize> = input
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        let employee_a = query[0] - 1;
        let employee_b = query[1] - 1;
        let ans = graph.lca(employee_a, employee_b);

        println!("{}", ans + 1);
    }
}
