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
    TreeNode *next; // problem #116
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

const int null = INT32_MAX;

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

            if (inputs[i] != null) {
                auto n = new TreeNode(inputs[i]);
                r->left = n;
                nodes.push(n);
            }

            i++;
            if (i<inputs.size()) {
                if (inputs[i] != null) {
                    auto n = new TreeNode(inputs[i]);
                    r->right = n;
                    nodes.push(n);
                }
            }
        }
    }
    return root;
}

TreeNode *find_node(TreeNode *root, int val) {
    if (!root || root->val == val) {
        return root;
    }
    auto l = find_node(root->left, val);
    if (l) {
        return l;
    }
    return find_node(root->right, val);
}

void print_tree(TreeNode *r) {
    if (!r) {
        printf("[]\n");
        return;
    }

    // bfs
    queue<TreeNode*> nodes;
    nodes.push(r);
    vector<TreeNode*> result;
    while (!nodes.empty()) {
        auto n = nodes.front();
        nodes.pop();
        result.push_back(n);
        if (n) {
            nodes.push(n->left);
            nodes.push(n->right);
        }
    }

    // remove trailing nulls
    int end = result.size() - 1;
    for (; end >= 0; end--) {
        if (result[end]) {
            break;
        }
    }

    // output
    printf("[");
    for (int i = 0; i <= end; i++) {
        if (result[i]) {
            printf("%d,", result[i]->val);
        }
        else {
            printf("null,");
        }
    }
    printf("\b]\n");
}

template<typename T>
void print_list(T* root) {
    if (!root) {
        printf("[]\n");
        return;
    }

    printf("[");
    while (root) {
        printf("%d,", root->val);
        root = root->next;
    }
    printf("\b]\n");
}

#endif