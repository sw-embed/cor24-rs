/* Sieve of Eratosthenes — from MakerLisp's COR24 C compiler */
/* Compiled with: cc24 sieve.c → as24 sieve.s → COR24 assembly */

int divdec[] = { 1000, 100, 10 };
char flags[8191];

putchr(c) { /* write char to UART */ }

printn(n)
{
    int i, d, q;

    for (i = 0; i < 3; i++) {
        q = 0;
        d = divdec[i];
        while (n >= d) {
            n = n - d;
            q = q + 1;
        }
        putchr(q + '0');
    }
    putchr(n + '0');
}

putstr(s)
char *s;
{
    while (*s)
        putchr(*s++);
}

main()
{
    int iter, i, k, prime, count;

    putstr("1000 iterations\n");

    for (iter = 1; iter <= 1000; iter++) {
        count = 0;

        for (i = 0; i <= 8190; i++)
            flags[i] = 1;

        for (i = 0; i <= 8190; i++) {
            if (flags[i]) {
                prime = i + i + 3;
                k = i + prime;
                while (k <= 8190) {
                    flags[k] = 0;
                    k = k + prime;
                }
                count = count + 1;
            }
        }
    }

    printn(count);
    putstr(" primes.\n");

    return 0;
}
