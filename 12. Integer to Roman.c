#include <stdio.h>

char * intToRoman(int num){
    char s[] = {'M', 'D', 'C', 'L', 'X', 'V', 'I'};
    int v[] = {1000, 500, 100, 50, 10, 5, 1};
    static char str[100] = {0};
    int j = 0, k = 0;

    for (int i=0; i<sizeof(v)/sizeof(v[0]); i++) {
        while (num >= v[i]) {
            str[j] = s[i];
            k++;
            j++;
            num -= v[i];
            if (k==4) {
                j -= 2;
                str[j-2] = s[i];
                str[j-1] = s[i-1];
                k = 0;
                if (j-3 >= 0 && str[j-3] == str[j-1]) {
                    str[j-3] = s[i];
                    str[j-2] = s[i-2];
                    j--;
                }
            }
        }
        k = 0;
    }
    str[j] = 0;
    return str;
}

int main() {
    int inputs[] = {19, 3999};
    for (int i=0; i<sizeof(inputs)/sizeof(inputs[0]); i++) {
        printf("%d: %s\n", inputs[i], intToRoman(inputs[i]));
    }
    return 0;
}