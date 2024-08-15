all: asan
	gcc -l:librust_ffi_lib.so  c_says_hello.c -o c_says_hello

asan:
	gcc -l:librust_ffi_lib.so  c_says_hello.c \
	-fsanitize=address -fsanitize=undefined -fno-sanitize-recover=all \
	-fsanitize=float-divide-by-zero -fsanitize=float-cast-overflow \
	-fno-sanitize=null -fno-sanitize=alignment \
	-o asan_c_says_hello

