#include <stdio.h>
#include "../stunc.h"

int main()
{
  COptions options = {0};
  options.software = "C client";
  char *result = NULL;
  int n = get_xor_mapped_address("plato-test.mantoux.org:3478", "3522", options, &result);
  if (n >= 0)
    printf("Result is %s\n", result);
  else
    printf("Error : %d\n", n);
  return n;
}