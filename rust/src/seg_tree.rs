struct SegTree {
    root: Node,
}

#[allow(dead_code)]
impl SegTree {
    fn new(values: &[i32]) -> Self {
        Self {
            root: Self::build_node(values, 0, values.len() - 1),
        }
    }

    fn build_node(values: &[i32], begin_index: usize, end_index: usize) -> Node {
        let mut node = Node::new(begin_index, end_index);

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

    fn update(&mut self, index: usize, value: i32) {
        Self::update_node(index, value, &mut self.root);
    }

    fn update_node(index: usize, value: i32, node: &mut Node) {
        if node.begin_index <= index && node.end_index >= index {
            if node.begin_index == node.end_index {
                node.min_value = value;
            } else {
                Self::update_node(index, value, node.left.as_mut().unwrap());
                Self::update_node(index, value, node.right.as_mut().unwrap());

                node.pull();
            }
        }
    }

    fn query(&self, begin_index: usize, end_index: usize) -> i32 {
        Self::query_node(begin_index, end_index, &self.root)
    }

    fn query_node(begin_index: usize, end_index: usize, node: &Node) -> i32 {
        if node.begin_index > end_index || node.end_index < begin_index {
            return i32::MAX;
        }
        if node.begin_index >= begin_index && node.end_index <= end_index {
            return node.min_value;
        }

        Self::query_node(begin_index, end_index, node.left.as_ref().unwrap()).min(Self::query_node(
            begin_index,
            end_index,
            node.right.as_ref().unwrap(),
        ))
    }
}

struct Node {
    begin_index: usize,
    end_index: usize,
    min_value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(begin_index: usize, end_index: usize) -> Self {
        Self {
            begin_index,
            end_index,
            min_value: 0,
            left: None,
            right: None,
        }
    }

    fn pull(&mut self) {
        self.min_value = self
            .left
            .as_ref()
            .unwrap()
            .min_value
            .min(self.right.as_ref().unwrap().min_value);
    }
}
