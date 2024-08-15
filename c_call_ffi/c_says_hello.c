#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rust_ffi_lib.h"

#define CHAR_ARR_SIZE 256

void c_hello_world()
{
    char *dest = calloc(CHAR_ARR_SIZE, sizeof(char));
    strncpy(dest, "Hello World c style!\n", 32);
    printf(dest);
    free(dest);
}

void rust_hello_world()
{
    const char *str = get_raw_hello_str();
    printf(str);
    // free_raw_str(str);
    printf("\n");
}

int main()
{
    c_hello_world();
    rust_hello_world();
    // rust_list_names();
    return 0;
}
