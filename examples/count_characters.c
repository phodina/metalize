#include <stdio.h>
#include <stdint.h>

#include "count_characters.h"

int main(int argc, char const *argv[])
{
	char *str = "Rust is awesome!";
  	uint32_t count = count_characters(str);
	printf("Count number of characters in '%s': %d\n", str, count);

	return 0;
}