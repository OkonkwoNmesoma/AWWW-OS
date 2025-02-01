#include <stdio.h>
int main() {
    int x = 5;
    printf("x before increment: %d\n", x);
    printf("Post-increment: %d\n", x++);
    printf("Pre-increment: %d\n", ++x);
    printf("Post-decrement: %d\n", x--);
    printf("Pre-decrement: %d\n", --x);
    return 0;
}