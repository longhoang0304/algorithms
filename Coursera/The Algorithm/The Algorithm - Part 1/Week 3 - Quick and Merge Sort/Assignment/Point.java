import java.util.Comparator;

import edu.princeton.cs.algs4.StdDraw;

public class Point implements Comparable<Point> {
  private int x;
  private int y;

  public Point(int x, int y) {
    this.x = x;
    this.y = y;
  }

  public void draw() {
    StdDraw.point(x, y);
  }

  public void drawTo(Point that) {
    StdDraw.line(this.x, this.y, that.x, that.y);
  }

  public String toString() {
    return "(" + x + ", " + y + ")";
  }

  public int compareTo(Point that) {
    if (y > that.y)
      return 1;
    if (y == that.y) {
      if (x > that.x)
        return 1;
      else
        return -1;
    }
    return -1;
  }

  public double slopeTo(Point that) {
    if (x == that.x) {
      if (y == that.y)
        return Double.NEGATIVE_INFINITY;
      return Double.POSITIVE_INFINITY;
    }
    if (y == that.y)
      return 0;
    return ((that.y - y) / (that.x - x));
  }

  private class SlopeOrder implements Comparator<Point> {
    public int compare(Point a, Point b) {
      double aSlope = slopeTo(a);
      double bSlope = slopeTo(b);
      if (aSlope < bSlope)
        return -1;
      if (bSlope > aSlope)
        return 1;
      return 0;
    }
  }

  public Comparator<Point> slopeOrder() {
    return new SlopeOrder();
  }
}
