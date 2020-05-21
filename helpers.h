#ifndef HELPERS_H
#define HELPERS_H

#include <cstdint>
#include <vector>
#include <queue>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

ListNode* parse_singly_linked_list(vector<int>& inputs) {
    ListNode list, *head = &list;
    for (auto& v : inputs) {
        head->next = new ListNode(v);
        head = head->next;
    }
    return list.next;
}

const int NULL_NODE = INT32_MAX;

TreeNode* parse_tree(vector<int>& inputs) {
    TreeNode* root = nullptr;
    queue<TreeNode*> nodes;
    for (int i=0; i<inputs.size(); i++) {
        if (!root) {
            root = new TreeNode(inputs[i]);
            nodes.push(root);
        }
        else {
            auto r = nodes.front();
            nodes.pop();

            if (inputs[i] != NULL_NODE) {
                auto n = new TreeNode(inputs[i]);
                r->left = n;
                nodes.push(n);
            }

            i++;
            if (i<inputs.size()) {
                if (inputs[i] != NULL_NODE) {
                    auto n = new TreeNode(inputs[i]);
                    r->right = n;
                    nodes.push(n);
                }
            }
        }
    }
    return root;
}

#endif