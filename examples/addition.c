#include <stdio.h>
#include <stdint.h>

#include "addition.h"

int main(int argc, char const *argv[])
{
	uint32_t sum = addition(1, 2);
  	printf("Adding 1 + 2 = %d\n", sum);

	return 0;
}