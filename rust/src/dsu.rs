use std::collections::HashMap;

struct Dsu {
    parent_or_sizes: Vec<i32>,
}

#[allow(dead_code)]
impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent_or_sizes: vec![-1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent_or_sizes[a] < 0 {
            return a;
        }

        self.parent_or_sizes[a] = self.find(self.parent_or_sizes[a] as usize) as i32;

        self.parent_or_sizes[a] as usize
    }

    fn union(&mut self, a: usize, b: usize) {
        let a_leader = self.find(a);
        let b_leader = self.find(b);
        if a_leader != b_leader {
            self.parent_or_sizes[a_leader] += self.parent_or_sizes[b_leader];
            self.parent_or_sizes[b_leader] = a_leader as i32;
        }
    }

    fn get_size(&mut self, a: usize) -> usize {
        let leader = self.find(a);

        -self.parent_or_sizes[leader] as usize
    }

    fn build_leader_to_group(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut leader_to_group = HashMap::new();
        for i in 0..self.parent_or_sizes.len() {
            leader_to_group
                .entry(self.find(i))
                .or_insert(Vec::new())
                .push(i);
        }

        leader_to_group
    }
}
