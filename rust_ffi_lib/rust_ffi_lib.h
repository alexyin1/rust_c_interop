#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct StringArray {
  const char *const *ptr;
  /**
   * manually set "len" to const
   */
  const size_t len;
} StringArray;

void free_raw_str(const char *s);

void free_raw_stringarray(struct StringArray array);

const char *get_raw_hello_str(void);

struct StringArray get_raw_names_vec(void);