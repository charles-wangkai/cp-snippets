struct LazySegTree {
    root: Node,
}

#[allow(dead_code)]
impl LazySegTree {
    fn new(values: &[i32]) -> Self {
        Self {
            root: Self::build_node(values, 0, values.len() - 1),
        }
    }

    fn build_node(values: &[i32], begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index, 0);

        if begin_index == end_index {
            node.min_value = values[begin_index];
        } else {
            let middle_index = (begin_index + end_index) / 2;
            node.left = Some(Box::new(Self::build_node(
                values,
                begin_index,
                middle_index,
            )));
            node.right = Some(Box::new(Self::build_node(
                values,
                middle_index + 1,
                end_index,
            )));

            node.pull();
        }

        node
    }

    fn update(&mut self, begin_index: usize, end_index: usize, delta: i32) {
        Self::update_node(begin_index, end_index, delta, &mut self.root);
    }

    fn update_node(begin_index: usize, end_index: usize, delta: i32, node: &mut Node) {
        if !(node.begin_index > end_index || node.end_index < begin_index) {
            if node.begin_index >= begin_index && node.end_index <= end_index {
                node.apply(delta);
            } else {
                node.push_down();

                Self::update_node(begin_index, end_index, delta, node.left.as_mut().unwrap());
                Self::update_node(begin_index, end_index, delta, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }

    fn query(&mut self, begin_index: usize, end_index: usize) -> i32 {
        Self::query_node(begin_index, end_index, &mut self.root)
    }

    fn query_node(begin_index: usize, end_index: usize, node: &mut Node) -> i32 {
        if node.begin_index > end_index || node.end_index < begin_index {
            return i32::MAX;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.get_computed_min_value();
        }

        node.push_down();

        node.pull();

        Self::query_node(begin_index, end_index, node.left.as_mut().unwrap()).min(Self::query_node(
            begin_index,
            end_index,
            node.right.as_mut().unwrap(),
        ))
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    delta: i32,
    min_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize, delta: i32) -> Self {
        Self {
            begin_index,
            end_index,
            delta,
            min_value: 0,
            left: None,
            right: None,
        }
    }

    fn get_computed_min_value(&self) -> i32 {
        self.min_value + self.delta
    }

    fn push_down(&mut self) {
        self.left.as_mut().unwrap().apply(self.delta);
        self.right.as_mut().unwrap().apply(self.delta);

        self.delta = 0;
    }

    fn apply(&mut self, d: i32) {
        self.delta += d;
    }

    fn pull(&mut self) {
        self.min_value = self
            .left
            .as_ref()
            .unwrap()
            .get_computed_min_value()
            .min(self.right.as_ref().unwrap().get_computed_min_value());
    }
}
