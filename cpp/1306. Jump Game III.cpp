#include <cstdio>
#include <vector>
#include <queue>
#include <map>

using namespace std;

class Solution {
public:
    bool canReach(vector<int> &arr, int start) {
        const int N = arr.size();
        queue<int> q;
        q.push(start);
        vector<bool> m(N, false);
        while(!q.empty()) {
            int i = q.front();
            if (arr[i] == 0) {
                return true;
            }
            q.pop();
            m[i] = true;
            if (i + arr[i] < N && !m[i + arr[i]]) {
                q.push(i + arr[i]);
            }
            if (i - arr[i] >= 0 && !m[i - arr[i]]) {
                q.push(i - arr[i]);
            }
        }
        return false;
    }
};

int main() {
    vector<int> digits = {4, 2, 3, 0, 3, 1, 2};
    const int n = 5;
    printf("%s\n", Solution().canReach(digits, n) ? "true" : "false");
    return 0;
}
