#include <cstdio>
#include <vector>
#include <algorithm>

#include "helpers.h"

using namespace std;

class Solution {
public:
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        vector<vector<int>> r;
        vector<TreeNode*> level;
        if (root) {level.push_back(root);}
        while (!level.empty()) {
            vector<int> l;
            vector<TreeNode*> next_level;
            for (auto n : level) {
                if (n) {
                    l.push_back(n->val);
                    if (n->left) {next_level.push_back(n->left);}
                    if (n->right) {next_level.push_back(n->right);}
                }
            }
            r.push_back(l);
            level = next_level;
        }
        reverse(r.begin(), r.end());
        return r;
    }
};

int main() {
    auto t = parse_tree({3,9,20,null,null,15,7});
    auto r = Solution().levelOrderBottom(t);
    printf("{\n");
    for (auto& l : r) {
        printf("  {");
        for (int n : l) {
            printf("%d,", n);
        }
        printf("\b}\n");
    }
    printf("}\n");
    return 0;
}