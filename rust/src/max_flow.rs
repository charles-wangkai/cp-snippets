use std::collections::VecDeque;

struct MaxFlow {
    edges: Vec<Edge>,
    edge_vecs: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl MaxFlow {
    fn new(size: usize) -> Self {
        Self {
            edges: Vec::new(),
            edge_vecs: vec![Vec::new(); size],
        }
    }

    fn add_edges(&mut self, u: usize, v: usize, cap: i32) {
        self.edges.push(Edge {
            from: u,
            to: v,
            capacity: cap,
        });
        self.edge_vecs[u].push(self.edges.len() - 1);

        self.edges.push(Edge {
            from: v,
            to: u,
            capacity: 0,
        });
        self.edge_vecs[v].push(self.edges.len() - 1);
    }

    fn dinic(&mut self, s: usize, t: usize) -> i32 {
        let mut result = 0;
        while let Some(mut levels) = self.bfs(s, t) {
            loop {
                let minflow = self.dfs(&mut levels, s, t, i32::MAX);
                if minflow == 0 {
                    break;
                }

                result += minflow;
            }
        }

        result
    }

    fn bfs(&mut self, s: usize, t: usize) -> Option<Vec<i32>> {
        let mut levels = vec![-1; self.edge_vecs.len()];
        levels[s] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(head) = queue.pop_front() {
            if head == t {
                return Some(levels);
            }

            for &e in &self.edge_vecs[head] {
                if self.edges[e].capacity != 0 && levels[self.edges[e].to] == -1 {
                    levels[self.edges[e].to] = levels[head] + 1;
                    queue.push_back(self.edges[e].to);
                }
            }
        }

        None
    }

    fn dfs(&mut self, levels: &mut [i32], s: usize, t: usize, low: i32) -> i32 {
        if s == t {
            return low;
        }

        let mut result = 0;
        for &e in &self.edge_vecs[s].to_vec() {
            if self.edges[e].capacity != 0 && levels[self.edges[e].to] == levels[s] + 1 {
                let next = self.dfs(
                    levels,
                    self.edges[e].to,
                    t,
                    (low - result).min(self.edges[e].capacity),
                );
                self.edges[e].capacity -= next;
                self.edges[e ^ 1].capacity += next;

                result += next;
                if result == low {
                    break;
                }
            }
        }

        if result == 0 {
            levels[s] = -1;
        }

        result
    }
}

#[allow(dead_code)]
struct Edge {
    from: usize,
    to: usize,
    capacity: i32,
}
