import scala.io.StdIn.readLine
import scala.collection.mutable.Map

var cache: Map[Int, Int] = Map()

def solve(n: Int): Int =
  if cache.contains(n) then
    return cache(n)

  val start = n
  var cycle = 0

  while n != 1 do
    cycle += 1

    if n % 2 == 0 then
      n /= 2
    else
      n = 3*n + 1

  cycle += 1
  cache(start) = cycle

end solve

@main def main(): Unit =
  print("N = ")
  var inp = readLine()
  var n = inp.toInt

  while n != 1 do
    print(s"$n ")
    if n % 2 == 0 then
      n /= 2
    else
      n = 3*n + 1

  println(s"$n ")
end main
