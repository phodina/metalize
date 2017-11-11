#include <stdio.h>
#include <stdint.h>

extern char *create_new_quote(uint8_t length);

extern void free_quote(char *);

int main(void) {

  char *quote = create_new_quote(5);
  printf("%s\n", quote);

  // TODO: Commenting this line calls the address sanitizer
  free_quote(quote);
}