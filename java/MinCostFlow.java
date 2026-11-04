import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;

class MinCostFlow {
  List<Edge> edges = new ArrayList<>();
  List<Integer>[] edgeLists;

  @SuppressWarnings("unchecked")
  MinCostFlow(int size) {
    edgeLists = new List[size];
    for (int i = 0; i < edgeLists.length; ++i) {
      edgeLists[i] = new ArrayList<>();
    }
  }

  void addEdges(int u, int v, int cap, int cost) {
    edges.add(new Edge(u, v, cap, cost));
    edgeLists[u].add(edges.size() - 1);

    edges.add(new Edge(v, u, 0, -cost));
    edgeLists[v].add(edges.size() - 1);
  }

  int computeMinCostByBellmanFord(int s, int t, int f) {
    int size = edgeLists.length;

    int result = 0;
    while (f != 0) {
      int[] prevEdges = new int[size];
      int[] distances = new int[size];
      Arrays.fill(distances, Integer.MAX_VALUE);
      distances[s] = 0;
      while (true) {
        boolean updated = false;
        for (int v = 0; v < size; ++v) {
          if (distances[v] != Integer.MAX_VALUE) {
            for (int e : edgeLists[v]) {
              Edge edge = edges.get(e);
              if (edge.capacity != 0 && distances[v] + edge.cost < distances[edge.to]) {
                distances[edge.to] = distances[v] + edge.cost;
                prevEdges[edge.to] = e;

                updated = true;
              }
            }
          }
        }

        if (!updated) {
          break;
        }
      }

      int d = f;
      for (int v = t; v != s; v = edges.get(prevEdges[v]).from) {
        d = Math.min(d, edges.get(prevEdges[v]).capacity);
      }
      f -= d;
      result += d * distances[t];

      for (int v = t; v != s; v = edges.get(prevEdges[v]).from) {
        Edge edge = edges.get(prevEdges[v]);

        edge.capacity -= d;
        edges.get(prevEdges[v] ^ 1).capacity += d;
      }
    }

    return result;
  }

  int computeMinCostFlowByDijkstra(int s, int t, int f) {
    int size = edgeLists.length;

    int result = 0;
    int[] h = new int[size];
    while (f != 0) {
      int[] prevEdges = new int[size];

      int[] distances = new int[size];
      Arrays.fill(distances, Integer.MAX_VALUE);
      distances[s] = 0;

      PriorityQueue<Element> pq = new PriorityQueue<>(Comparator.comparing(e -> e.distance));
      pq.offer(new Element(s, 0));

      while (!pq.isEmpty()) {
        Element head = pq.poll();
        if (head.distance <= distances[head.index]) {
          for (int e : edgeLists[head.index]) {
            Edge edge = edges.get(e);
            if (edge.capacity != 0
                && distances[head.index] + edge.cost + h[head.index] - h[edge.to]
                    < distances[edge.to]) {
              distances[edge.to] = distances[head.index] + edge.cost + h[head.index] - h[edge.to];
              prevEdges[edge.to] = e;

              pq.offer(new Element(edge.to, distances[edge.to]));
            }
          }
        }
      }

      for (int i = 0; i < h.length; ++i) {
        h[i] += distances[i];
      }

      int d = f;
      for (int v = t; v != s; v = edges.get(prevEdges[v]).from) {
        d = Math.min(d, edges.get(prevEdges[v]).capacity);
      }
      f -= d;
      result += d * h[t];

      for (int v = t; v != s; v = edges.get(prevEdges[v]).from) {
        Edge edge = edges.get(prevEdges[v]);

        edge.capacity -= d;
        edges.get(prevEdges[v] ^ 1).capacity += d;
      }
    }

    return result;
  }

  static class Edge {
    int from;
    int to;
    int capacity;
    int cost;

    Edge(int from, int to, int capacity, int cost) {
      this.from = from;
      this.to = to;
      this.capacity = capacity;
      this.cost = cost;
    }
  }

  static class Element {
    int index;
    int distance;

    Element(int index, int distance) {
      this.index = index;
      this.distance = distance;
    }
  }
}
