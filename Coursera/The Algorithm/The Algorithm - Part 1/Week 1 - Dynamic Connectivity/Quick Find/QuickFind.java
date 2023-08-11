import java.util.Scanner;

class QuickFind {
  private int[] id;
  QuickFind(int n) {
    id = new int[n];
    for (int i = 0; i < n; i++) {
      id[i] = i;
    }
  }

  public boolean isConnected(int a, int b) {
    return this.id[a] == this.id[b];
  }

  public void connect(int a, int b) {
    int idb = this.id[b];
    int ida = this.id[a];

    if (idb == ida) return;
    for (int i = 0; i < this.id.length; i++) {
      if (this.id[i] == ida) this.id[i] = idb;
    }
  }
}

class Main {
  public static void main(String[] args) {
    QuickFind qf = new QuickFind(10);
    int a, b;
    int ops = -1;
    Scanner sc = new Scanner(System.in);

    System.out.print("Graph with 10 nodes inited\n");
    while (ops != 0) {
      System.out.println(">> ");
      ops = sc.nextInt();
      if (ops == 0) break;
      a = sc.nextInt();
      b = sc.nextInt();
      if (ops == 1) qf.connect(a, b);
      if (ops == 2) System.out.println(a + " can connect to " + b + " : " + qf.isConnected(a, b) + "\n"); 
    }

    sc.close();
  }
}
