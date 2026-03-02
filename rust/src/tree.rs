#[allow(dead_code)]
struct Tree {
    n: usize,
    u: Vec<usize>,
    v: Vec<usize>,
    root: usize,
    edge_vecs: Vec<Vec<usize>>,
    depths: Vec<i32>,
    ancestors: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Tree {
    fn new(u: &[usize], v: &[usize], root: usize) -> Self {
        let n = u.len() + 1;

        let mut edge_vecs = vec![Vec::new(); n];
        for i in 0..u.len() {
            edge_vecs[u[i]].push(i);
            edge_vecs[v[i]].push(i);
        }

        let mut tree = Self {
            n,
            u: u.to_vec(),
            v: v.to_vec(),
            root,
            edge_vecs,
            depths: vec![0; n],
            ancestors: vec![vec![usize::MAX; (n.ilog2() as usize) + 1]; n],
        };
        tree.init(0, usize::MAX, root);

        tree
    }

    fn init(&mut self, depth: i32, parent: usize, node: usize) {
        self.depths[node] = depth;

        self.ancestors[node][0] = parent;
        for i in 1..self.ancestors[node].len() {
            if self.ancestors[node][i - 1] != usize::MAX {
                self.ancestors[node][i] = self.ancestors[self.ancestors[node][i - 1]][i - 1];
            }
        }

        for edge in self.edge_vecs[node].clone() {
            let adj = if node == self.u[edge] {
                self.v[edge]
            } else {
                self.u[edge]
            };

            if adj != parent {
                self.init(depth + 1, node, adj);
            }
        }
    }

    fn find_lca(&self, mut node1: usize, mut node2: usize) -> usize {
        if self.depths[node1] < self.depths[node2] {
            return self.find_lca(node2, node1);
        }

        for i in (0..self.ancestors[node1].len()).rev() {
            if self.ancestors[node1][i] != usize::MAX
                && self.depths[self.ancestors[node1][i]] >= self.depths[node2]
            {
                node1 = self.ancestors[node1][i];
            }
        }

        if node1 == node2 {
            return node1;
        }

        for i in (0..self.ancestors[0].len()).rev() {
            if self.ancestors[node1][i] != self.ancestors[node2][i] {
                node1 = self.ancestors[node1][i];
                node2 = self.ancestors[node2][i];
            }
        }

        self.ancestors[node1][0]
    }
}
