#include <random>
#include <numeric>
#include <cstdio>
#include <vector>

using namespace std;

class Solution {
    random_device rd;
    mt19937 gen;
    discrete_distribution<> d;
public:
    Solution(vector<int>& w) {
        gen = mt19937(rd());
        d = discrete_distribution<>(w.begin(), w.end());
    }

    int pickIndex() {
        return d(gen);
    }
};

int main() {
    vector<int> weights = {3, 14, 1, 7}, ints(weights.size());
    Solution s(weights);
    const int trials = 1000000, weight_sum = accumulate(weights.begin(), weights.end(), 0);
    for (int i=0; i<trials; i++) {
        ints[s.pickIndex()]++;
    }
    printf("index\tweight\tactual weight\toccurrences\n");
    for (int i=0; i<ints.size(); i++) {
        printf("%d\t%6d\t%13.3f\t%11d\n", i, weights[i], weight_sum*ints[i]/double(trials), ints[i]);
    }
    return 0;
}