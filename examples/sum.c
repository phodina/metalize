#include <stdio.h>
#include <stdint.h>

extern uint32_t sum(const uint32_t *numbers, size_t length);

int main(void) {
  uint32_t numbers[6] = {1,2,3,4,5,6};
  uint32_t s = sum(numbers, 6);
  printf("Sum of array: %d\n", s);
  return 0;
}