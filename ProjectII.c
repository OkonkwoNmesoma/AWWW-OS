#include <stdio.h>

int main() {
    int experience, age;
    long salary;

    // Input experience and age
    printf("Enter experience (in years): ");
    scanf("%d", &experience);
    
    printf("Enter age: ");
    scanf("%d", &age);

    // Calculate salary based on conditions
    if (experience >= 3) {  // Experienced person
        if (age >= 40) {
            salary = 560000;
        } else if (age >= 30) {
            salary = 480000;
        } else if (age >= 28) {
            salary = 300000;
        } else {
            salary = 100000;  // Not experienced or age below 28
        }
    } else {  // Inexperienced person
        salary = 100000;
    }

    // Output the salary
    printf("The salary is: N%ld\n", salary);

    return 0;
}
