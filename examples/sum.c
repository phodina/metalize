#include <stdio.h>
#include <stdint.h>

#include "sum.h"

int main(void) {
  uint32_t numbers[6] = {1,2,3,4,5,6};
  uint32_t s = sum(numbers, 6);
  printf("Sum of array: %d\n", s);
  return 0;
}