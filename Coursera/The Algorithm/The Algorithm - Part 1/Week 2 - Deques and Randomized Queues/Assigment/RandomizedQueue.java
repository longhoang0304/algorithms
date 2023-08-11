import java.util.Iterator;
import java.util.NoSuchElementException;

import edu.princeton.cs.algs4.StdRandom;

public class RandomizedQueue<Item> implements Iterable<Item> {
  private class PackedArray {
    private int[] bits;
    private int setBits = 0;

    PackedArray(int size) {
      bits = new int[(int) Math.ceil(size / 32.0)];
    }

    public int size() {
      return setBits;
    }

    public void set(int idx) {
      if (check(idx))
        return;
      int page = idx / 32;
      if (page >= bits.length)
        throw new IndexOutOfBoundsException();

      int pageIdx = idx % 32;
      int bit = bits[page];

      bit |= 1 << pageIdx;
      bits[page] = bit;
      setBits += 1;
    }

    public boolean check(int idx) {
      int page = idx / 32;
      if (page >= bits.length)
        throw new IndexOutOfBoundsException();

      int pageIdx = idx % 32;
      int bit = bits[page];

      return (bit >> pageIdx & 0x1) == 1;
    }
  }

  private Item[] data;
  private int qSize = 0;

  // construct an empty randomized queue
  public RandomizedQueue() {
    data = (Item[]) new Object[1];
  }

  private void resize(int newSize) {
    Item[] newData = (Item[]) new Object[newSize];
    for (int i = 0; i < qSize; i++)
      newData[i] = data[i];
    data = newData;
  }

  private void swapItem(int idxa, int idxb) {
    if (idxb < 0) {
      return;
    }
    Item tmp = data[idxa];
    data[idxa] = data[idxb];
    data[idxb] = tmp;
  }

  // is the randomized queue empty?
  public boolean isEmpty() {
    return qSize == 0;
  }

  // return the number of items on the randomized queue
  public int size() {
    return qSize;
  }

  // add the item
  public void enqueue(Item item) {
    if (item == null)
      throw new IllegalArgumentException("Item must be not null");
    data[qSize++] = item;
    if (qSize >= data.length)
      resize(data.length * 2);
  }

  // remove and return a random item
  public Item dequeue() {
    if (isEmpty())
      throw new NoSuchElementException("queue is empty");
    int idx = StdRandom.uniformInt(qSize);
    Item item = data[idx];

    swapItem(idx, --qSize);
    data[qSize] = null;

    if (qSize > 0 && qSize == data.length / 4)
      resize(data.length / 2);
    return item;
  }

  // return a random item (but do not remove it)
  public Item sample() {
    if (isEmpty())
      throw new NoSuchElementException("queue is empty");
    int idx = StdRandom.uniformInt(size());
    return data[idx];
  }

  private class ListIterator implements Iterator<Item> {
    private PackedArray savedIdx = new PackedArray(qSize);

    @Override
    public boolean hasNext() {
      return qSize != 0 && savedIdx.size() < qSize;
    }

    @Override
    public Item next() {
      if (!hasNext())
        throw new NoSuchElementException("queue is empty");
      int idx;
      do {
        idx = StdRandom.uniformInt(qSize);
      } while (savedIdx.check(idx));

      savedIdx.set(idx);
      return data[idx];
    }

    @Override
    public void remove() {
      throw new UnsupportedOperationException();
    }
  }

  // return an independent iterator over items in random order
  public Iterator<Item> iterator() {
    Iterator<Item> it = new ListIterator();
    return it;
  }

  // unit testing (required)
  public static void main(String[] args) {
    RandomizedQueue<Integer> q = new RandomizedQueue<Integer>();
    System.out.println("Queue is empty? " + q.isEmpty());
    for (int i = 0; i < 10; i++) {
      q.enqueue(i);
    }
    System.out.println("Queue is empty? " + q.isEmpty());
    System.out.println("Queue size? " + q.size());

    Iterator<Integer> it = q.iterator();
    System.out.println("Series 1: ");
    while (it.hasNext()) {
      System.out.println(it.next());
    }

    it = q.iterator();
    System.out.println("Series 2: ");
    while (it.hasNext()) {
      System.out.println(it.next());
    }

    System.out.println("Series 3: ");
    q.dequeue();
    for (int i = 0; i < q.size(); i++) {
      System.out.println(q.sample());
    }
  }

}