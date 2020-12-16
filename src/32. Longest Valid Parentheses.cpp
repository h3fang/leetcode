#include <cstdio>
#include <string>
#include <vector>
#include <stack>

using namespace std;

class Solution {
  public:
    int longestValidParentheses2(string s) {
        const int N = s.size();
        int r = 0;
        stack<int> st; st.push(-1);
        for (int i = 0; i < N; i++) {
            if (s[i] == '(') {
                st.push(i);
            }
            else {
                st.pop();
                if (st.empty()) {
                    st.push(i);
                }
                else {
                    r = max(r, i-st.top());
                }
            }
        }
        return r;
    }

    int longestValidParentheses(string s) {
        const int N = s.size();
        int r = 0;
        vector<int> dp(N);
        for (int i = 1; i < N; i++) {
            if (s[i] == ')') {
                if (s[i-1] == '(') {
                    dp[i] = (i>=2 ? dp[i-2] : 0) + 2;
                }
                else {
                    if (i-dp[i-1]-1 >= 0 && s[i-dp[i-1]-1] == '(') {
                        dp[i] = dp[i-1] + 2;
                        if (i-dp[i-1]-2 >= 0) {
                            dp[i] += dp[i-dp[i-1]-2];
                        }
                    }
                }
                r = r < dp[i] ? dp[i] : r;
            }
        }

        return r;
    }
};

int main() {
    string s = ")()())";
    // string s = ")(((((()())()()))()(()))(";
    printf("%d\n", Solution().longestValidParentheses2(s));
    return 0;
}