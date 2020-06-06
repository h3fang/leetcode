#include <cstdio>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    void deleteNode(ListNode *node) {
        *node = *node->next;
    }
};

int main() {
    const int val = 3;
    vector<int> inputs = {1, 2, 3, 4, 5};
    auto l = parse_singly_linked_list(inputs), h = l;
    while (h) {
        if (h->val == val) {
            break;
        }
        h = h->next;
    }

    Solution().deleteNode(h);
    while (l) {
        printf("%d ", l->val);
        l = l->next;
    }
    printf("\n");
    return 0;
}