#include "helpers.h"
#include <cassert>

class Solution {
public:
    ListNode *reverseList(ListNode *head) {
        ListNode *prev = nullptr;
        while (head) {
            auto next = head->next;
            head->next = prev;
            prev = head;
            head = next;
        }
        return prev;
    }
};

int main() {
    auto head = parse_singly_linked_list({1, 2, 3, 4, 5});
    auto expected = parse_singly_linked_list({5, 4, 3, 2, 1});

    auto r = Solution().reverseList(head);

    while (expected) {
        assert(r && expected->val == r->val);
        r = r->next;
        expected = expected->next;
    }
    assert(!r);
}
