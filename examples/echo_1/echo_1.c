#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char** argv) {
	int aval = 1024;
	int used = 0;
	char* buf = malloc(used);
	if (argc < 2) {
		return 0;
	}
	for (int i = 1; i < argc; i++) {
		int len = strlen(argv[i]);
		if (used + len > aval) {
			aval *= 2;
			buf = realloc(buf, aval);
		}
		strcpy(buf + used, argv[i]);
		used += strlen(argv[i]) + 1;
		if (i != argc - 1) {
			buf[used] = ' ';
		}
	}
	printf("%s\n", buf);
	free(buf);
}
