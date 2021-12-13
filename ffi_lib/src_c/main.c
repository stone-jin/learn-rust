#include <stdio.h>
#include "ffi_lib.h"

int main(){
  printf("hello world");
  print_hello_from_rust();
}