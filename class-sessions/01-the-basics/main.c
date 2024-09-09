// to run this script, run the following command: 
// gcc -o target/main.o src/main.c && ./target/main.o
#include <stdio.h>

typedef struct
{
    int age;
    float gpa;
    char *name;
} student_S;

student_S ayush;

void init()
{
    ayush.age = 21;
    ayush.gpa = 0.00;
    ayush.name = "Ayush";
}

void display_student(student_S *student)
{
    if (student != NULL && student->name != NULL)
    {
        printf(
            "%s is %d years old and has a gpa of %f\n",
            student->name,
            student->age,
            student->gpa);
    }
}

int main()
{
    init();
    display_student(&ayush);
}