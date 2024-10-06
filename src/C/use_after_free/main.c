#include <stdio.h>
#include <stdlib.h>

int main() {
	int* pointer = malloc(sizeof(int));
	*pointer = 5;
	pointer = NULL;
	free(pointer);
	*pointer = 6; 
}
