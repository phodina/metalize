#include <stdio.h>
#include <stdint.h>

typedef struct {
  uint32_t x;
  uint32_t y;
} tuple_t;

extern tuple_t swap_point_values(tuple_t);

int main(void) {
  tuple_t initial = { .x = 10, .y = 20 };
  printf("Point pre-state: x = %d, y = %d\n", initial.x, initial.y);
  tuple_t new = swap_point_values(initial);
  printf("Point post-state: x = %d, y = %d\n", new.x, new.y);
  return 0;
}