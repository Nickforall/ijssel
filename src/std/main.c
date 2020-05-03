#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void print_digit(long number) 
{
    printf("%ld\n", number);
}

void ijssel_exit(long number) 
{
    exit(number);
}

long ijssel_time()
{
    return time(NULL);
}