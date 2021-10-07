#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if (p && q) {
            if (p->val != q->val) {
                return false;
            }
            return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
        }
        return !p && !q;
    }
};

int main() {
    const vector<int> nums1 = {1,2,1}, nums2 = {1,1,2};
    auto t1 = parse_tree(nums1), t2 = parse_tree(nums2);
    printf("%d\n", Solution().isSameTree(t1, t2));
    return 0;
}