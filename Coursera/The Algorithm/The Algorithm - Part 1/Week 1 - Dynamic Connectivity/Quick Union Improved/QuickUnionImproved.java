import java.util.Scanner;

class QuickUnion {
  private int[] id;
  private int[] sz;
  QuickUnion(int n) {
    id = new int[n];
    sz = new int[n];
    for (int i = 0; i < n; i++) {
      id[i] = i;
      sz[i] = 1;
    }
  }

  public boolean isConnected(int a, int b) {
    return this.findRoot(a) == this.findRoot(b);
  }

  int findRoot(int id) {
    int root = id;
    while (this.id[root] != root) {
      // path compression
      this.id[root] = this.id[this.id[root]];
      root = this.id[root];
    }
    return root;
  }

  public void connect(int a, int b) {
    int roota = this.findRoot(a);
    int rootb = this.findRoot(b);
    if (roota == rootb) return;
    int sizea = this.sz[roota];
    int sizeb = this.sz[rootb];
    // smaller tree will be merged with bigger tree
    if (sizea < sizeb) {
      this.id[roota] = rootb;
      this.sz[rootb] += sizea;
      return;
    }
    this.id[rootb] = roota;
    this.sz[roota] += sizeb;
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
