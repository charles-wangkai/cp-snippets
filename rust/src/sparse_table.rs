struct SparseTable {
    st: Vec<Vec<i32>>,
    operator: fn(i32, i32) -> i32,
}

#[allow(dead_code)]
impl SparseTable {
    fn new(values: &[i32], operator: fn(i32, i32) -> i32) -> Self {
        let size = (values.len().ilog2() as usize) + 1;
        let mut st = vec![vec![0; size]; values.len()];
        for i in 0..values.len() {
            st[i][0] = values[i];
        }
        for exponent in 1..size {
            for i in 0..values.len() {
                if i + (1 << exponent) <= values.len() {
                    st[i][exponent] = operator(
                        st[i][exponent - 1],
                        st[i + (1 << (exponent - 1))][exponent - 1],
                    );
                }
            }
        }

        Self { st, operator }
    }

    fn query(&self, begin_index: usize, end_index: usize) -> i32 {
        let exponent = (end_index - begin_index + 1).ilog2() as usize;

        (self.operator)(
            self.st[begin_index][exponent],
            self.st[end_index + 1 - (1 << exponent)][exponent],
        )
    }
}
