include!("scc.rs");

struct TwoSat {
    n: usize,
    scc: Scc,
}

#[allow(dead_code)]
impl TwoSat {
    fn new(n: usize) -> Self {
        Self {
            n,
            scc: Scc::new(2 * n),
        }
    }

    fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.scc.add_edge(
            2 * i + (if f { 0 } else { 1 }),
            2 * j + (if g { 1 } else { 0 }),
        );
        self.scc.add_edge(
            2 * j + (if g { 0 } else { 1 }),
            2 * i + (if f { 1 } else { 0 }),
        );
    }

    fn find_assignment(&self) -> Option<Vec<bool>> {
        let components = self.scc.build_components();

        let mut assignment = vec![false; self.n];
        for i in 0..assignment.len() {
            if components[2 * i] == components[2 * i + 1] {
                return None;
            }

            assignment[i] = components[2 * i] < components[2 * i + 1];
        }

        Some(assignment)
    }
}
