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
    printf("%s\n", str);
    free_raw_str(str);
}

void rust_list_names() {
    StringArray myarr = get_raw_names_vec();
    for (int i=0; i<myarr.len; i++) {
        printf("Name is: %s\n", myarr.ptr[i]);
    }
    free_raw_stringarray(myarr);
}

void print_namespace(StringArray ns) {
    // printf("Here!");
    if (ns.len == 0) {
        return;
    }
    if (ns.len == 1) {
        printf("%s\n", ns.ptr[0]);
        return;
    }
    for (int i=0; i<ns.len-1; i++) {
        // "." seperates namespaces
        printf("%s.", ns.ptr[i]);
    }
    printf("%s\n", ns.ptr[ns.len-1]);
}

void print_namespaces(NamespaceIdentArray ns_arr) {
    if (ns_arr.len == 0) {
        printf("no namespaces!");
        return;
    } 
    // printf("ns_arr.len == %i", ns_arr.len);
    for (int i=0; i<ns_arr.len; i++) {
        print_namespace(ns_arr.ptr[i]);
    }
}

int main()
{
    c_hello_world();
    rust_hello_world();
    rust_list_names();
    printf("CCCCC\n");
    NamespaceIdentArray ns_arr = list_namespaces();
    // printf("XD %s", ns_arr.ptr[0].ptr[0]);
    print_namespaces(ns_arr);
    free_namespaces_vec(ns_arr);
    return 0;
}
