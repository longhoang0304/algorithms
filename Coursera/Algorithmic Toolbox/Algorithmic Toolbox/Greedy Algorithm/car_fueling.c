#include <stdio.h>
#include <stdlib.h>


int main(void) {
  int dist, max_mile, no_stops;
  scanf("%d", &dist);
  scanf("%d", &max_mile);
  scanf("%d", &no_stops);

  if (max_mile >= dist) {
    puts("0");
    return 0;
  }

  int current_stop = 0;
  int next_stop = current_stop + max_mile;
  int stop;
  int needed_stops = 0;
  int last_stop = 0;

  while( no_stops-- ) {
    scanf("%d", &stop);
    if (stop - last_stop > max_mile) {
      puts("-1");
      return 0;
    }
    if (stop >= next_stop) {
      needed_stops += 1;
      current_stop = stop == next_stop ? stop : last_stop;
      next_stop = current_stop + max_mile;
    }
    last_stop = stop;
  }

  stop = dist;
  if (stop - last_stop > max_mile) {
    puts("-1");
    return 0;
  }

  needed_stops += stop > next_stop;
  printf("%d", needed_stops);

  return 0;
}
