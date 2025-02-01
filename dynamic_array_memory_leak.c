#include <stdio.h>
#include <stdlib.h>

struct Person {
    char name[50];
    int age;
};

int main() {
    int n = 3;  // Number of persons
    struct Person *people;

    people = (struct Person *)malloc(n * sizeof(struct Person));

    if (people == NULL) {
        printf("Memory allocation failed.\n");
        return 1;
    }

    for (int i = 0; i < n; i++) {
        printf("Enter name of person %d: ", i + 1);
        scanf("%s", people[i].name);
        printf("Enter age of person %d: ", i + 1);
        scanf("%d", &people[i].age);
    }

    printf("\nEntered details:\n");
    for (int i = 0; i < n; i++) {
        printf("Person %d: Name: %s, Age: %d\n", i + 1, people[i].name, people[i].age);
    }

    return 0;
}
