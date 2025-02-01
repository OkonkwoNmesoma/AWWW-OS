#include <stdio.h>

void addVectors(int* vec1, int* vec2, int* result, int size) {
    for (int i = 0; i < size; ++i) {
        *(result + i) = *(vec1 + i) + *(vec2 + i); // Adding using pointers
    }
}

int main() {
    const int size = 3;  // Size of the vectors
    int vec1[] = {2, 4, 7};
    int vec2[] = {4, 9, 1};
    int result[size];

    // Add the two vectors
    addVectors(vec1, vec2, result, size);

    // Display the result
    printf("Resultant vector: ");
    for (int i = 0; i < size; ++i) {
        printf("%d ", *(result + i)); // Accessing result array using pointers
    }
    printf("\n");

    return 0;
}
