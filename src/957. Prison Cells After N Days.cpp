#include <bitset>
#include <cstdio>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<int> prisonAfterNDays(vector<int> &cells, int N) {
        const int M = 8;

        bitset<M> c(0);
        for (int i = 0; i < M; i++) {
            c[i] = cells[i];
        }

        unordered_map<bitset<M>, bitset<M>> states;

        bitset<M> n(0);

        int k = 0;
        for (; k < N; k++) {
            if (states.find(c) != states.end()) {
                break;
            }
            for (int i = 1; i < M - 1; i++) {
                n[i] = c[i - 1] == c[i + 1];
            }
            states[c] = n;
            c = n;
        }

        if (k < N) {
            n = c;
            int period = 1;

            while (true) {
                if (states[n] == c) {
                    break;
                } else {
                    period++;
                    n = states[n];
                }
            }

            period = (N-k) % period;
            for (int i = 0; i < period; i++) {
                c = states[c];
            }
        }

        for (int i = 0; i < M; i++) {
            cells[i] = c[i];
        }

        return cells;
    }
};

int main() {
    vector<int> cells = {1, 0, 0, 1, 0, 0, 1, 0};
    const int N = 1000000000;
    printf("[");
    for (int i : Solution().prisonAfterNDays(cells, N)) {
        printf("%d,", i);
    }
    printf("\b]\n");
    return 0;
}