#include <cstdio>
#include <vector>
#include <stack>

using namespace std;

class Solution {
public:
    int trap_dp(vector<int>& height) {
        const int N = height.size();
        if (N == 0) {return 0;}
        vector<int> left(N+1), right(N+1);
        left[0] = height[0];
        right[N-1] = height[N-1];
        for (int i=1; i<N; i++) {
            left[i] = max(left[i-1], height[i]);
        }
        for (int i=N-2; i>=0; i--) {
            right[i] = max(right[i+1], height[i]);
        }
        int r = 0;
        for (int i=0; i<N; i++) {
            r += min(left[i], right[i]) - height[i];
        }
        return r;
    }

    int trap_stack(vector<int>& height) {
        const int N = height.size();
        if (N == 0) {return 0;}
        stack<int> s;
        int r = 0;
        for (int i = 0; i < height.size(); i++) {
            while (!s.empty() && height[i] > height[s.top()]) {
                int c = s.top();
                s.pop();
                if (s.empty()) {
                    break;
                }
                int d = i - s.top() - 1;
                r += d*(min(height[s.top()], height[i]) - height[c]);
            }
            s.push(i);
        }
        return r;
    }

    int trap(vector<int>& height) {
        const int N = height.size();
        if (N == 0) {return 0;}
        int res = 0, l = 0, r = N-1, l_max = 0, r_max = 0;
        while (l < r) {
            if (height[l] < height[r]) {
                if (height[l] > l_max) {
                    l_max = height[l];
                } else {
                    res += l_max - height[l];
                }
                l++;
            } else {
                if (height[r] > r_max) {
                    r_max = height[r];
                } else {
                    res += r_max - height[r];
                }
                r--;
            }
        }
        return res;
    }
};

int main() {
    vector<int> height = {0,1,0,2,1,0,1,3,2,1,2,1};
    printf("%d\n", Solution().trap_stack(height));
    return 0;
}