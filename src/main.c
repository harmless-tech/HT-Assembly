#include <stdio.h>
#include <parson.h>
#include <binn.h>
#include <pthread.h>

int main(int argc, char *argv[]) {
	printf("Hello!\n");
	
	// Args
	printf("Arg Amount: %d\n", argc);
	for(int i = 0; i < argc; i++)
		printf("Arg %d: %s\n", i, argv[i]);
	//
	
	return 0;
}