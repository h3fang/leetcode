#include <cstdio>
#include <vector>

using namespace std;

class Solution {
    vector<vector<char>> memo;

public:
    bool canPartition_topdown(vector<int> &nums) {
        int sum = 0;
        for (int n : nums) {
            sum += n;
        }
        if (sum % 2 != 0) {
            return false;
        }
        memo.resize(nums.size(), vector<char>(sum / 2, -1));
        return dfs(nums, nums.size(), sum / 2);
    }

    bool dfs(const vector<int> &nums, int n, int remaining) {
        if (remaining == 0) {
            return true;
        }
        if (n == 0 || remaining < 0) {
            return false;
        }
        if (memo[n - 1][remaining - 1] != -1) {
            return memo[n - 1][remaining - 1] == 1;
        }
        bool result = dfs(nums, n - 1, remaining - nums[n - 1]) || dfs(nums, n - 1, remaining);
        memo[n - 1][remaining - 1] = result ? 1 : 0;
        return result;
    }

    bool canPartition_bottomup(vector<int> &nums) {
        const int M = nums.size();
        if ( M <= 1) {
            return false;
        }
        int sum = 0;
        for (int n : nums) {
            sum += n;
        }
        if (sum % 2 != 0) {
            return false;
        }
        const int array_sum = sum / 2;
        vector<bool> dp(array_sum + 1, false);
        dp[0] = true;
        for (const int n : nums) {
            for (int j = array_sum; j >= n; j--) {
                dp[j] = dp[j] || dp[j - n];
            }
        }
        return dp[array_sum];
    }
};

int main() {
    vector<int> nums = {1,2,5};
    printf("%s\n", Solution().canPartition_bottomup(nums) ? "true" : "false");
    return 0;
}
