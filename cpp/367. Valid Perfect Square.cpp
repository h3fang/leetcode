#include <stdio.h>

bool isPerfectSquare(int num){
    long a = 1, b = num;
    while (true) {
        long c = (a+b)/2;
        if (c*c == num) {
            return true;
        }
        else if (c == a || c == b) {
            return false;
        }
        else if (c*c < num) {
            a = c;
        }
        else if (c*c > num) {
            b = c;
        }
    }
    return false;
}

int main() {
    int inputs[] = {1,14};

    for (int i=0; i<sizeof(inputs)/sizeof(inputs[0]); i++) {
        printf("%d %d \n", inputs[i], isPerfectSquare(inputs[i]));
    }
    return 0;
}