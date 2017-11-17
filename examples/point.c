#include <stdio.h>
#include <stdint.h>

#include "point.h"

int main(void) {
  Tuple initial = { .x = 10, .y = 20 };
  printf("Point pre-state: x = %d, y = %d\n", initial.x, initial.y);
  Tuple new = swap_point_values(initial);
  printf("Point post-state: x = %d, y = %d\n", new.x, new.y);
  return 0;
}