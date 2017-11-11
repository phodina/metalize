#include <stdio.h>
#include <stdint.h>

extern uint32_t count_characters(const char *str);

int main(int argc, char const *argv[])
{
	char *str = "Rust is awesome!";
  	uint32_t count = count_characters(str);
	printf("Count number of characters in '%s': %d\n", str, count);

	return 0;
}