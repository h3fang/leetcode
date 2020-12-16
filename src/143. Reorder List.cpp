#include <cstdio>
#include <vector>
#include <queue>

#include "helpers.h"

using namespace std;

class Solution {
public:
    void reorderList(ListNode* head) {
        deque<ListNode*> nodes;
        auto h = head;
        while (h) {
            nodes.push_back(h);
            h = h->next;
        }
        const int N = nodes.size();
        ListNode d;
        h = &d;
        while (!nodes.empty()) {
            h->next = nodes.front();
            nodes.pop_front();
            h = h->next;
            if (!nodes.empty()) {
                h->next = nodes.back();
                nodes.pop_back();
                h = h->next;
            }
        }
        h->next = nullptr;
        head = d.next;
    }
};

int main() {
    vector<int> nodes = {1,2,3,4,5};
    auto r = parse_singly_linked_list(nodes);
    Solution().reorderList(r);
    print_list(r);
    return 0;
}