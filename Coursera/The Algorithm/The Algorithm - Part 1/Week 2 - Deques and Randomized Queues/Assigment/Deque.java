import java.util.Iterator;
import java.util.NoSuchElementException;

public class Deque<Item> implements Iterable<Item> {
  private class Node {
    Node next;
    Node prev;
    Item value;

    Node(Item item) {
      value = item;
    }
  }

  private  class LinkedList implements Iterable<Item> {
    private Node head;
    private Node tail;
    private int llSize;

    LinkedList() {
      llSize = 0;
    }

    public boolean isEmpty() {
      return llSize == 0;
    }

    public int size() {
      return llSize;
    }

    public void addFirst(Item item) {
      Node n = new Node(item);

      n.next = head;
      if (head != null)
        head.prev = n;
      head = n;
      llSize += 1;

      if (llSize == 1)
        tail = head;
    }

    public void addLast(Item item) {
      Node n = new Node(item);

      n.prev = tail;
      if (tail != null)
        tail.next = n;
      tail = n;
      llSize += 1;

      if (llSize == 1)
        head = tail;
    }

    public Item removeFirst() {
      if (isEmpty())
        throw new NoSuchElementException("queue is empty");
      llSize -= 1;

      Item item = head.value;
      head = head.next;
      if (head != null) head.prev = null;

      if (llSize == 0)
        head = tail = null;
      return item;
    }

    public Item removeLast() {
      if (isEmpty())
        throw new NoSuchElementException("queue is empty");
      llSize -= 1;

      Item item = tail.value;
      tail = tail.prev;
      if (tail != null) tail.next = null;

      if (llSize == 0)
        head = tail = null;
      return item;
    }

    private class ListIterator implements Iterator<Item> {
      private Node curr = head;

      public boolean hasNext() {
        return curr != null;
      }

      public void remove() {
        throw new UnsupportedOperationException();
      }

      public Item next() {
        if (!hasNext()) {
          throw new NoSuchElementException();
        }
        Item item = curr.value;
        curr = curr.next;
        return item;
      }
    }

    public Iterator<Item> iterator() {
      return new ListIterator();
    }

  }

  private LinkedList data;

  // construct an empty deque
  public Deque() {
    data = new LinkedList();
  }

  // is the deque empty?
  public boolean isEmpty() {
    return data.isEmpty();
  }

  // return the number of items on the deque
  public int size() {
    return data.size();
  }

  // add the item to the front
  public void addFirst(Item item) {
    if (item == null)
      throw new IllegalArgumentException("Item must be not null");
    data.addFirst(item);
  }

  // add the item to the back
  public void addLast(Item item) {
    if (item == null)
      throw new IllegalArgumentException("Item must be not null");
    data.addLast(item);
  }

  // remove and return the item from the front
  public Item removeFirst() {
    return data.removeFirst();
  }

  // remove and return the item from the back
  public Item removeLast() {
    return data.removeLast();
  }

  // return an iterator over items in order from front to back
  public Iterator<Item> iterator() {
    Iterator<Item> it = new Iterator<Item>() {

      private Iterator<Item> it = data.iterator();

      @Override
      public boolean hasNext() {
        return it.hasNext();
      }

      @Override
      public Item next() {
        return it.next();
      }

      @Override
      public void remove() {
        throw new UnsupportedOperationException();
      }
    };
    return it;
  }

  // unit testing (required)
  public static void main(String[] args) {
    Deque<Integer> dq = new Deque<Integer>();
    System.out.println("Queue is empty? " + dq.isEmpty());
    for (int i = 0; i < 10; i++) {
      if (i % 2 == 0)
        dq.addFirst(i);
      else
        dq.addLast(i);
    }
    System.out.println("Queue is empty? " + dq.isEmpty());
    System.out.println("Queue size? " + dq.size());

    System.out.println("Series");
    Iterator<Integer> it = dq.iterator();
    while (it.hasNext()) {
      System.out.println(it.next());
    }

    dq.removeFirst();
    dq.removeLast();

    System.out.println("Series");
    it = dq.iterator();
    while (it.hasNext()) {
      System.out.println(it.next());
    }
  }

}