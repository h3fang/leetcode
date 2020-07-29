#include <cstdio>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    int leastInterval(vector<char> &tasks, int n) {
        unordered_map<char, int> ts;
        for (char t : tasks) {
            ts[t]++;
        }
        int max_freq = 0, n_max_freq = 0;
        for (auto [k,v] : ts) {
            max_freq = max(max_freq, v);
        }
        for (auto [k,v] : ts) {
            if (v == max_freq) {
                n_max_freq++;
            }
        }

        int slots = (max_freq - 1) * (n - n_max_freq + 1);
        int remaining = tasks.size() - (n_max_freq) * max_freq;
        int idles = max(slots - remaining, 0);

        return tasks.size() + idles;
    }
};

int main() {
    vector<char> tasks = {'A','A','A','A','A','A','B','C','D','E','F','G'};
    const int n = 2;
    printf("%d\n", Solution().leastInterval(tasks, n));
    return 0;
}
