#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    TreeNode *dfs(vector<int> &nums, int i, int j) {
        if (i >= j) {
            return nullptr;
        }
        int m = i + (j - i) / 2;
        auto l = dfs(nums, i, m);
        auto r = dfs(nums, m + 1, j);
        return new TreeNode(nums[m], l, r);
    }

public:
    TreeNode *sortedArrayToBST(vector<int> &nums) {
        int n = nums.size();
        return dfs(nums, 0, n);
    }
};

void in_order(TreeNode *root, vector<int> &nums) {
    if (root) {
        in_order(root->left, nums);
        nums.push_back(root->val);
        in_order(root->right, nums);
    }
}

int main() {
    vector<int> nums = {-10, -3, 0, 5, 9};
    auto r = Solution().sortedArrayToBST(nums);
    vector<int> p;
    in_order(r, p);
    assert(nums == p);
    return 0;
}
