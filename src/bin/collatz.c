#include <stdio.h>

int main() {
    long long longest_num = 1;
    long long longest_length = 1;

    for (long long n = 1; n <= 1000000; n++) {
        long long x = n;
        long long counter = 1;

        while (x > 1) {
            counter++;
            if (x % 2 == 0) {
                x = x / 2;
            } else {
                x = 3 * x + 1;
            }
        }

        if (counter > longest_length) {
            longest_length = counter;
            longest_num = n;
        }
    }

    printf("%lld\n", longest_num);

    return 0;
}
