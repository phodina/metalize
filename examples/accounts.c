#include <stdio.h>
#include <stdint.h>

#include "accounts.h"

int main(void) {
  AccountDB *database = account_database_new();

  char *user1 = "90210";
  char *user2 = "20500";

  account_database_populate(database);
  uint32_t pop1 = account_database_current_balance(database, user1);
  uint32_t pop2 = account_database_current_balance(database, user2);

  account_database_free(database);

  printf("ID: %s Money: %d\n", user1, pop1);
  printf("ID: %s Money: %d\n", user2, pop2);
  printf("Difference in money %d\n", pop1 - pop2);
}
