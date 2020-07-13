#ifndef HELPERS_H
#define HELPERS_H

#include <cstdio>
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

ListNode* parse_singly_linked_list(const vector<int>& inputs) {
    ListNode list, *head = &list;
    for (auto& v : inputs) {
        head->next = new ListNode(v);
        head = head->next;
    }
    return list.next;
}

const int NULL_NODE = INT32_MAX;

TreeNode* parse_tree(const vector<int>& inputs) {
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

void print_tree(TreeNode *r) {
    queue<TreeNode*> nodes;
    if (!r) {
        printf("[]\n");
    }
    nodes.push(r);
    printf("[%d,", r->val);
    while (!nodes.empty()) {
        auto n = nodes.front();
        nodes.pop();
        if (!n->left && !n->right) {
            continue;
        }
        if (!n->left) {
            printf("null,");
        }
        else {
            printf("%d,", n->left->val);
            nodes.push(n->left);
        }
        if (!n->right) {
            printf("null,");
        }
        else {
            printf("%d,", n->right->val);
            nodes.push(n->right);
        }
    }
    printf("\b]\n");
}

template<typename T>
void print_list(T* root) {
    printf("[");
    while (root) {
        printf("%d,", root->val);
        root = root->next;
    }
    printf("\b]\n");
}

#endif