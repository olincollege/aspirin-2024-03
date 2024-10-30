#include <stdint.h>
#include <stdio.h>

void mystery_function()
{
}

int main(void)
{
    uint8_t x = 100;
    uint8_t y = 20;

    mystery_function();

    printf("x + y = %i\n", x + y);
}