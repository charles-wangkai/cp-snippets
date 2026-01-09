class SegTree {
  Node root;

  SegTree(int[] values) {
    root = buildNode(values, 0, values.length - 1);
  }

  private Node buildNode(int[] values, int beginIndex, int endIndex) {
    Node node = new Node(beginIndex, endIndex);

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

  void update(int index, int value) {
    update(index, value, root);
  }

  private void update(int index, int value, Node node) {
    if (node.beginIndex <= index && node.endIndex >= index) {
      if (node.beginIndex == node.endIndex) {
        node.minValue = value;
      } else {
        update(index, value, node.left);
        update(index, value, node.right);

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
      return node.minValue;
    }

    return Math.min(
        query(beginIndex, endIndex, node.left), query(beginIndex, endIndex, node.right));
  }

  static class Node {
    int beginIndex;
    int endIndex;
    int minValue;
    Node left;
    Node right;

    Node(int beginIndex, int endIndex) {
      this.beginIndex = beginIndex;
      this.endIndex = endIndex;
    }

    void pull() {
      minValue = Math.min(left.minValue, right.minValue);
    }
  }
}
