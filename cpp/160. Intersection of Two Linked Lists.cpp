#include "helpers.h"
#include <cassert>

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        auto a = headA;
        auto b = headB;
        while (true) {
            if (a == b) {
                return a;
            }
            if (a) {
                a = a->next;
            } else {
                a = headB;
            }
            if (b) {
                b = b->next;
            } else {
                b = headA;
            }
        }
    }
};

int main() {
    auto headA = parse_singly_linked_list({4, 1});
    auto headB = parse_singly_linked_list({5, 6, 1});
    auto c = parse_singly_linked_list({8, 4, 5});

    for (auto h : {headA, headB}) {
        while (h->next) {
            h = h->next;
        }
        h->next = c;
    }

    auto r = Solution().getIntersectionNode(headA, headB);
    assert(r != nullptr);
    assert(r->val == 8);
}
