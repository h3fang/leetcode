#include <cstdio>
#include <queue>
#include <map>
#include <algorithm>

#include "helpers.h"

using namespace std;

class Solution {
public:
    vector<vector<int>> verticalTraversal(TreeNode* root) {
        vector<vector<int>> r;
        queue<pair<TreeNode*, pair<int, int>>> q;
        map<int, vector<pair<int, int>>> result;
        if (root) {
            q.push({root, {0, 0}});
        }

        while (!q.empty()) {
            auto [node, pos] = q.front();
            q.pop();

            result[pos.first].push_back({node->val, pos.second});

            if (node->left) {
                q.push({node->left, {pos.first - 1, pos.second + 1}});
            }
            if (node->right) {
                q.push({node->right, {pos.first + 1, pos.second + 1}});
            }
        }

        for (auto& [k, v] : result) {
            sort(v.begin(), v.end(), [](const auto& l, const auto& r){
                if (l.second == r.second) {
                    return l.first < r.first;
                } else {
                    return l.second < r.second;
                }
            });
            vector<int> line;
            for (auto& n : v) {
                line.push_back(n.first);
            }
            r.push_back(line);
        }
        return r;
    }
};

int main() {
    vector<int> nodes = {3,9,20,null,null,15,7};
    auto root = parse_tree(nodes);
    auto r = Solution().verticalTraversal(root);

    if (r.empty()) {
        printf("[]\n");
    }

    printf("[\n");
    for (auto& line : r) {
        printf("  [");
        for (int n : line) {
            printf("%d,", n);
        }
        printf("\b]\n");
    }
    printf("]\n");

    return 0;
}