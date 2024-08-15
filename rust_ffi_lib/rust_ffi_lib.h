#ifndef rust_ffi_lib
#define rust_ffi_lib

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


/**
 * c struct for NamespaceIdent.inner()
 */
typedef struct StringArray {
  const char *const *ptr;
  /**
   * manually set "len" to const
   */
  size_t len;
} StringArray;

/**
 * c struct for Vec<NamespaceIdent>
 * an array of stringarrays
 */
typedef struct NamespaceIdentArray {
  const struct StringArray *ptr;
  /**
   * manually set "len" to const
   */
  size_t len;
} NamespaceIdentArray;

void free_namespaces_vec(struct NamespaceIdentArray array);

void free_raw_str(const char *s);

void free_raw_stringarray(struct StringArray array);

const char *get_raw_hello_str(void);

struct StringArray get_raw_names_vec(void);

struct NamespaceIdentArray list_namespaces(void);

#endif /* rust_ffi_lib */
