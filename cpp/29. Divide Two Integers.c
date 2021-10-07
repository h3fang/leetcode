#include <stdio.h>
#include <stdint.h>

int divide(int dividend, int divisor){
    int64_t a = dividend < 0 ? -(int64_t)dividend : dividend;
    int64_t b = divisor < 0 ? -(int64_t)divisor : divisor;
    int64_t r = 0;
    if (a == b) {r=1; goto out;}
    if (b == 1) {r=a; goto out;}
    while (b < a) {
        for (int i=1; i<32; i++) {
            int64_t d = b << i;
            if (d > a) {
                d = d >> 1;
                a -= d;
                r += 1 << (i-1);
                break;
            }
        }
    }

out:
    if ((dividend > 0) != (divisor > 0)) {
        r = -r;
    }
    if (r > INT32_MAX) {return INT32_MAX;}
    if (r < INT32_MIN) {return INT32_MIN;}
    return r;
}

int main() {
    int a = INT32_MIN, b = -1;
    printf("%d\n", divide(a, b));
    return 0;
}