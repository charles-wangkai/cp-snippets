class LazySegTree {
  Node root;

  LazySegTree(int[] values) {
    root = buildNode(values, 0, values.length - 1);
  }

  private Node buildNode(int[] values, int beginIndex, int endIndex) {
    Node node = new Node(beginIndex, endIndex, 0);

    if (beginIndex == endIndex) {
      node.minValue = values[beginIndex];
    } else {
      int middleIndex = (beginIndex + endIndex) / 2;
      node.left = buildNode(values, beginIndex, middleIndex);
      node.right = buildNode(values, middleIndex + 1, endIndex);

      node.pull();
    }

    return node;
  }

  void update(int beginIndex, int endIndex, int delta) {
    update(beginIndex, endIndex, delta, root);
  }

  private void update(int beginIndex, int endIndex, int delta, Node node) {
    if (!(node.beginIndex > endIndex || node.endIndex < beginIndex)) {
      if (node.beginIndex >= beginIndex && node.endIndex <= endIndex) {
        node.apply(delta);
      } else {
        node.pushDown();

        update(beginIndex, endIndex, delta, node.left);
        update(beginIndex, endIndex, delta, node.right);

        node.pull();
      }
    }
  }

  int query(int beginIndex, int endIndex) {
    return query(beginIndex, endIndex, root);
  }

  private int query(int beginIndex, int endIndex, Node node) {
    if (node.beginIndex > endIndex || node.endIndex < beginIndex) {
      return Integer.MAX_VALUE;
    }
    if (node.beginIndex >= beginIndex && node.endIndex <= endIndex) {
      return node.getComputedMinValue();
    }

    node.pushDown();

    node.pull();

    return Math.min(
        query(beginIndex, endIndex, node.left), query(beginIndex, endIndex, node.right));
  }

  static class Node {
    int beginIndex;
    int endIndex;
    int delta;
    int minValue;
    Node left;
    Node right;

    Node(int beginIndex, int endIndex, int delta) {
      this.beginIndex = beginIndex;
      this.endIndex = endIndex;
      this.delta = delta;
    }

    int getComputedMinValue() {
      return minValue + delta;
    }

    void pushDown() {
      left.apply(delta);
      right.apply(delta);

      delta = 0;
    }

    void apply(int d) {
      delta += d;
    }

    void pull() {
      minValue = Math.min(left.getComputedMinValue(), right.getComputedMinValue());
    }
  }
}
