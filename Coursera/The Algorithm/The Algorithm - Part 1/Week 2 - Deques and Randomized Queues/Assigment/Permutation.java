import edu.princeton.cs.algs4.StdIn;
import edu.princeton.cs.algs4.StdOut;

public class Permutation {
  public static void main(String[] args) {
    if (args.length < 1) {
      System.out.println("Missing argument");
      return;
    }
    int n = Integer.parseInt(args[0]);
    RandomizedQueue<String> rq = new RandomizedQueue<String>();
    while (!StdIn.isEmpty()) {
      rq.enqueue(StdIn.readString());
  }
    while (n-- > 0) {
      System.out.println(rq.dequeue());
    }
  }
}