#include <stdio.h>

int main() {
    char input_char;
    
    // Prompt the user for a character between A to J
    printf("Enter a character between A and J: ");
    scanf("%c", &input_char);
    
    // Check if the character is between A and J
    if (input_char >= 'A' && input_char <= 'J') {
        printf("The next 6 characters are: ");
        // Print the next 6 characters
        for (int i = 1; i <= 6; i++) {
            printf("%c ", input_char + i);
        }
        printf("\n");
    } else {
        printf("Invalid input. Please enter a character between A and J.\n");
    }

    return 0;
}
