import java.util.Scanner;

class QuickUnion {
  private int[] id;
  QuickUnion(int n) {
    id = new int[n];
    for (int i = 0; i < n; i++) {
      id[i] = i;
    }
  }

  public boolean isConnected(int a, int b) {
    return this.findRoot(a) == this.findRoot(b);
  }

  int findRoot(int id) {
    int root = id;
    while (this.id[root] != root) root = this.id[root];
    return root;
  }

  public void connect(int a, int b) {
    int roota = this.findRoot(a);
    this.id[b] = roota;
  }
}

class Main {
  public static void main(String[] args) {
    QuickUnion qu = new QuickUnion(10);
    int a, b;
    int ops = -1;
    Scanner sc = new Scanner(System.in);

    System.out.println("Graph with 10 nodes inited!");
    while (ops != 0) {
      System.out.print(">> ");
      ops = sc.nextInt();
      if (ops == 0) break;
      a = sc.nextInt();
      b = sc.nextInt();
      if (ops == 1) qu.connect(a, b);
      if (ops == 2) System.out.println(a + " can connect to " + b + " : " + qu.isConnected(a, b) + "\n"); 
    }

    sc.close();
  }
}
