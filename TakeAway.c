#include <stdio.h>

int main() {
    char choice;
    
    // Ask user to input the type of data
    printf("Enter the type of input (c for char, f for float, i for int): ");
    scanf(" %c", &choice);

    // Using switch statement to handle different data types
    switch(choice) {
        case 'c': {
            char ch;
            printf("Enter a character: ");
            scanf(" %c", &ch);

            // Print the next four characters and ASCII codes
            printf("The next four characters are:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%c (ASCII: %d)\n", ch + i, ch + i);
            }
            
            // Print the size of the operator
            printf("Size of character operator: %zu bytes\n", sizeof(ch));
            break;
        }

        case 'f': {
            float num;
            printf("Enter a float number: ");
            scanf("%f", &num);

            // Print the next four floats in multiples of 3
            printf("The next four float numbers (multiples of 3) are:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%.2f\n", num + (i * 3));
            }

            // Print the size of the operator
            printf("Size of float operator: %zu bytes\n", sizeof(num));
            break;
        }

        case 'i': {
            int num;
            printf("Enter an integer: ");
            scanf("%d", &num);

            // Print the next four integers in multiples of 3
            printf("The next four integers (multiples of 3) are:\n");
            for (int i = 1; i <= 4; i++) {
                printf("%d\n", num + (i * 3));
            }

            // Print the size of the operator
            printf("Size of integer operator: %zu bytes\n", sizeof(num));
            break;
        }

        default:
            printf("Invalid choice.\n");
    }

    return 0;
}
