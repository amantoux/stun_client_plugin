#include <stdio.h>
#include "../../native/target/binding.h"

int main()
{
  printf("Testing get_xor_mapped_address...\n");
  COptions options = {0};
  options.software = "C client";
  Response response = get_xor_mapped_address("plato-test.mantoux.org:3478", options);
  if (response.status >= 0)
    printf("Result is %s\n", response.value);
  else
    printf("Error : %s\n", response.error);
  return 0;
}