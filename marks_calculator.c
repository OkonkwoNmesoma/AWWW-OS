#include <stdio.h>

int main() {
    float csc201, csc205, sta205, total, average, percentage;

    // Input marks for each subject
    printf("Enter marks for CSC 201: ");
    scanf("%f", &csc201);
    
    printf("Enter marks for CSC 205: ");
    scanf("%f", &csc205);

    printf("Enter marks for STA 205: ");
    scanf("%f", &sta205);

    // Calculate total, average, and percentage
    total = csc201 + csc205 + sta205;
    average = total / 3;
    percentage = (total / 300) * 100;

    // Display results
    printf("\nTotal Marks: %.2f\n", total);
    printf("Average Marks: %.2f\n", average);
    printf("Percentage: %.2f%%\n", percentage);

    return 0;
}
