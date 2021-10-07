#include <cstdio>
#include <vector>
#include <string>
#include <queue>
#include <stack>

#include "helpers.h"

using namespace std;

// class BSTIterator {
//     queue<TreeNode*> q;
// public:
//     BSTIterator(TreeNode* root) {
//         inorder(root);
//     }

//     void inorder(TreeNode* root) {
//         if (root) {
//             inorder(root->left);
//             q.push(root);
//             inorder(root->right);
//         }
//     }

//     int next() {
//         auto root = q.front();
//         q.pop();
//         return root->val;
//     }

//     bool hasNext() {
//         return !q.empty();
//     }
// };

class BSTIterator {
    stack<TreeNode*> s;

public:
    BSTIterator(TreeNode* root) {
        inorder(root);
    }

    void inorder(TreeNode* root) {
        while (root) {
            s.push(root);
            root = root->left;
        }
    }

    int next() {
        auto root = s.top();
        s.pop();
        if (root->right) {
            inorder(root->right);
        }
        return root->val;
    }

    bool hasNext() {
        return !s.empty();
    }
};

int main() {
    vector<int> nodes = {7, 3, 15, null, null, 9, 20};
    auto root = parse_tree(nodes);
    auto it = BSTIterator(root);
    vector<string> operators = {"BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"};
    for (const auto& op : operators) {
        if (op == "next") {
            printf("%d,", it.next());
        }
        else if (op == "hasNext") {
            printf("%s,", it.hasNext() ? "true" : "false");
        }
        else {
            printf("null,");
        }
    }
    printf("\b \n");
    return 0;
}