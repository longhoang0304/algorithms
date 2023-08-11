import edu.princeton.cs.algs4.WeightedQuickUnionUF;

/**
 * Percolation
 */
public class Percolation {
  private static final byte[] X = {1, -1, 0};
  private static final byte[] Y = {1, -1, 0};

  private WeightedQuickUnionUF uf;
  private boolean[][] grid;
  private int size;
  private int opendedSites = 0;
  private int top;
  private int bottom;

  // creates n-by-n grid, with all sites initially blocked
  public Percolation(int n) {
    if (n <= 0) throw new IllegalArgumentException("grid size must be positive but " + n + " was found!");
    this.grid = new boolean[n][n];
    this.size = n;
    this.uf = new WeightedQuickUnionUF(n * n + 2);
    this.top = n * n;
    this.bottom = n * n + 1;
    this.opendedSites = 0;

  }

  // opens the site (row, col) if it is not open already
  public void open(int row, int col) {
    if (this.isOpen(row, col)) return;
    int id = calculateCellId(row, col);

    this.grid[row - 1][col - 1] = true;
    opendedSites += 1;

    if (row == 1) this.uf.union(id, top);
    if (row == size) this.uf.union(id, bottom);
    unionLocal(row, col);
    
  }

  // is the site (row, col) open?
  public boolean isOpen(int row, int col) {
    validate(row, col);
    return this.grid[row - 1][col - 1];
  }

  // is the site (row, col) full?
  public boolean isFull(int row, int col) {
    if (!this.isOpen(row, col)) return false;
    int id = calculateCellId(row, col);
    int curRoot = this.uf.find(id);
    int topRoot = this.uf.find(top);

    return curRoot == topRoot;
  }

  // returns the number of open sites
  public int numberOfOpenSites() {
    return this.opendedSites;
  }

  // does the system percolate?
  public boolean percolates() {
    int topRoot = this.uf.find(top);
    int btmRoot = this.uf.find(bottom);
    return topRoot == btmRoot;
  }

  private void unionLocal(int row, int col) {
    int id = calculateCellId(row, col);
    for (int x : Percolation.X) {
      for (int y: Percolation.Y) {
        if (Math.abs(x) == Math.abs(y)) continue;

        int nx = row + x;
        int ny = col + y;
        if (!this.isInGrid(nx, ny) || !this.isOpen(nx, ny)) continue;

        int nid = calculateCellId(nx, ny);
        this.uf.union(id, nid);
      }
    }
  }

  private boolean isInGrid(int row, int col) {
    if (this.size < row || this.size < col) return false;
    if (1 > row || 1 > col) return false;
    return true;
  }

  private int calculateCellId(int row, int col) {
    row -= 1;
    col -= 1;
    return row * size + col;
  }

  private void validate(int row, int col) {
    if (!isInGrid(row, col)) throw new IllegalArgumentException();
  }

  // test client (optional)
  public static void main(String[] args) {
    // this code has a backwash problem
    // fix it here: https://github.com/allegoricalJest/coursera-percolation
  }
}