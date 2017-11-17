#include <stdio.h>
#include <stdint.h>

#include "quotes.h"

int main(void) {

  char *quote = create_new_quote(5);
  printf("%s\n", quote);

  // TODO: Commenting this line calls the address sanitizer
  free_quote(quote);
}