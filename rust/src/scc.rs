struct Scc {
    adj_vecs: Vec<Vec<usize>>,
    reversed_adj_vecs: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Scc {
    fn new(n: usize) -> Self {
        Self {
            adj_vecs: vec![Vec::new(); n],
            reversed_adj_vecs: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adj_vecs[from].push(to);
        self.reversed_adj_vecs[to].push(from);
    }

    fn topological_sort(&self) -> Vec<usize> {
        let n = self.adj_vecs.len();

        let mut sorted = Vec::new();
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                self.search1(&mut sorted, &mut visited, i);
            }
        }
        sorted.reverse();

        sorted
    }

    fn search1(&self, sorted: &mut Vec<usize>, visited: &mut [bool], node: usize) {
        visited[node] = true;

        for &adj in &self.adj_vecs[node] {
            if !visited[adj] {
                self.search1(sorted, visited, adj);
            }
        }

        sorted.push(node);
    }

    fn build_components(&self) -> Vec<usize> {
        let n = self.adj_vecs.len();

        let sorted = self.topological_sort();

        let mut components = vec![usize::MAX; n];
        let mut component = 0;
        for node in sorted {
            if components[node] == usize::MAX {
                self.search2(&mut components, component, node);
                component += 1;
            }
        }

        components
    }

    fn search2(&self, components: &mut [usize], component: usize, node: usize) {
        components[node] = component;

        for &adj in &self.reversed_adj_vecs[node] {
            if components[adj] == usize::MAX {
                self.search2(components, component, adj);
            }
        }
    }
}
