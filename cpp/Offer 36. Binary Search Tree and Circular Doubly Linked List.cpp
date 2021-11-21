#include "helpers.h"

using namespace std;

using Node = TreeNode;

class Solution {
    Node* head = nullptr;
    Node* tail = nullptr;
public:
    Node *treeToDoublyList(Node *root) {
        if (!root) {
            return root;
        }
        dfs(root);
        head->left = tail;
        tail->right = head;
        return head;
    }

    void dfs(Node* root) {
        if (!root) {return;}
        dfs(root->left);
        if (tail) {
            tail->right = root;
        } else {
            head = root;
        }
        root->left = tail;
        tail = root;
        dfs(root->right);
    }
};

int main() {
    auto root = parse_tree({4, 2, 5, 1, 3});
    auto result = Solution().treeToDoublyList(root);
    printf("%d->", result->val);
    auto h = result->right;
    while (h != result) {
        printf("%d->", h->val);
        h = h->right;
    }
    printf("\b\b  \n");
}
