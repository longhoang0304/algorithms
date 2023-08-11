import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;
import edu.princeton.cs.algs4.StdStats;

public class PercolationStats {
  private static final double P95 = 1.96;
  private int percolationSize;
  private double[] trialRecords;
  private int trialRounds;

  // perform independent trials on an n-by-n grid
  public PercolationStats(int n, int trials) {
    if (n <= 0 || trials <= 0) throw new IllegalArgumentException("n and trials must be > 0");
    this.percolationSize = n;
    this.trialRecords = new double[trials];
    this.trialRounds = trials;
    this.monteCarloSimulation();
  }

  private boolean openRandomBlock(Percolation p) {
    int rol = StdRandom.uniformInt(percolationSize) + 1;
    int col = StdRandom.uniformInt(percolationSize) + 1;
    p.open(rol, col);
    if (rol == 0) return true;
    if (col == percolationSize - 1) return true;
    return false;
  }

  private double monteCarloSimulationTest() {
    Percolation p = new Percolation(this.percolationSize);
    while (!p.percolates()) this.openRandomBlock(p);
    return (p.numberOfOpenSites() * 1.0 / (percolationSize * percolationSize));
  }

  private void monteCarloSimulation() {
    for (int i = 0; i < this.trialRounds; i++) {
      this.trialRecords[i] = this.monteCarloSimulationTest();
    }
  }

  // sample mean of percolation threshold
  public double mean() {
    return StdStats.mean(trialRecords);
  }

  // sample standard deviation of percolation threshold
  public double stddev() {
    return StdStats.stddev(trialRecords);
  }

  // low endpoint of 95% confidence interval
  public double confidenceLo() {
    double xmu = StdStats.mean(trialRecords);
    double std = StdStats.stddev(trialRecords);

    return xmu - (PercolationStats.P95 * std / Math.sqrt(this.trialRounds));
  }

  // high endpoint of 95% confidence interval
  public double confidenceHi() {
    double xmu = StdStats.mean(trialRecords);
    double std = StdStats.stddev(trialRecords);

    return xmu + (PercolationStats.P95 * std / Math.sqrt(this.trialRounds));
  }

   // test client (see below)
   public static void main(String[] args) {
      int gridSize = 10;
      int trialCount = 10;
      if (args.length >= 2) {
          gridSize = Integer.parseInt(args[0]);
          trialCount = Integer.parseInt(args[1]);
      }
      PercolationStats ps = new PercolationStats(gridSize, trialCount);

      String confidence = ps.confidenceLo() + ", " + ps.confidenceHi();
      StdOut.println("mean                    = " + ps.mean());
      StdOut.println("stddev                  = " + ps.stddev());
      StdOut.println("95% confidence interval = [" + confidence + "]");
   }
}
