class TwoSat {
  int n;
  Scc scc;

  TwoSat(int n) {
    this.n = n;
    scc = new Scc(2 * n);
  }

  void addClause(int i, boolean f, int j, boolean g) {
    scc.addEdge(2 * i + (f ? 0 : 1), 2 * j + (g ? 1 : 0));
    scc.addEdge(2 * j + (g ? 0 : 1), 2 * i + (f ? 1 : 0));
  }

  boolean[] findAssignment() {
    int[] components = scc.buildComponents();

    boolean[] assignment = new boolean[n];
    for (int i = 0; i < assignment.length; ++i) {
      if (components[2 * i] == components[2 * i + 1]) {
        return null;
      }

      assignment[i] = components[2 * i] < components[2 * i + 1];
    }

    return assignment;
  }
}
