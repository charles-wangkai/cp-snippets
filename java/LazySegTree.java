class LazySegTree {
  Node root;

  LazySegTree(int[] values) {
    root = buildNode(values, 0, values.length - 1);
  }

  private Node buildNode(int[] values, int beginIndex, int endIndex) {
    if (beginIndex == endIndex) {
      return new Node(beginIndex, endIndex, 0, values[beginIndex], null, null);
    }

    int middleIndex = (beginIndex + endIndex) / 2;
    Node left = buildNode(values, beginIndex, middleIndex);
    Node right = buildNode(values, middleIndex + 1, endIndex);

    return new Node(beginIndex, endIndex, 0, Math.min(left.minValue, right.minValue), left, right);
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

    Node(int beginIndex, int endIndex, int delta, int minValue, Node left, Node right) {
      this.beginIndex = beginIndex;
      this.endIndex = endIndex;
      this.delta = delta;
      this.minValue = minValue;
      this.left = left;
      this.right = right;
    }

    int getComputedMinValue() {
      return minValue + delta;
    }

    void pushDown() {
      if (delta != 0) {
        left.apply(delta);
        right.apply(delta);

        delta = 0;
      }
    }

    void apply(int d) {
      delta += d;
    }

    void pull() {
      minValue = Math.min(left.getComputedMinValue(), right.getComputedMinValue());
    }
  }
}
