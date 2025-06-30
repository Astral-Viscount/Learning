#include <stdio.h>

int main(void) 
{
    printf("Hello, World!\n");

    printf("for loop \n");
    for (int i = 0; i < 5; i++) 
    {
        printf("%i \n", i);
    }

    printf("while loop\n");

    int j = 5;
    while (j > 0) 
    {
        printf("%i\n", j);
        j--;
    }

    printf("do while loop\n");

    int num;
    do
    {
        printf("Positive Num: ");
        scanf("%i", &num);
    } while (num < 1);

    
}
