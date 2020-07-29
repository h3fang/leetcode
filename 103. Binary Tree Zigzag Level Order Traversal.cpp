#include <cstdio>
#include <vector>
#include <algorithm>

#include "helpers.h"

using namespace std;

class Solution {
public:
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<vector<int>> r;
        vector<TreeNode*> level = {root};
        bool left_to_right = true;
        while (!level.empty()) {
            vector<TreeNode*> next;
            vector<int> nodes;
            for (auto n : level) {
                if (n) {
                    nodes.push_back(n->val);
                    if (left_to_right) {
                        next.push_back(n->left);
                        next.push_back(n->right);
                    } else {
                        next.push_back(n->right);
                        next.push_back(n->left);
                    }
                }
            }
            if (!nodes.empty()) r.push_back(nodes);
            reverse(next.begin(), next.end());
            level = next;
            left_to_right = !left_to_right;
        }
        return r;
    }
};

int main() {
    vector<int> nodes = {3,9,20,NULL_NODE,NULL_NODE,15,7};
    auto root = parse_tree(nodes);
    for(const auto& level : Solution().zigzagLevelOrder(root)) {
        for (int n : level) {
            printf("%d,", n);
        }
        printf("}\n");
    }
    return 0;
}
