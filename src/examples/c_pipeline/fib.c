/* Fibonacci — from Luther Johnson's COR24 C compiler */
/* Compiled with: cc24 fib.c → as24 fib.s → COR24 assembly */

int fib(int n)
{
    if (n < 2)
        return 1;

    return fib(n - 1) + fib(n - 2);
}

int main()
{
    int result;

    printf("Fibonacci 10\n");

    result = fib(10);

    printf("%d\n", result);

    return 0;
}
