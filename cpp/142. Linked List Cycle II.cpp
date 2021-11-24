#include "helpers.h"
#include <cassert>

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        auto slow = head;
        auto fast = head;
        if (!head || !head->next) {
            return nullptr;
        }
        while (true) {
            slow = slow->next;
            fast = fast->next->next;
            if (!slow || !fast || !fast->next) {
                return nullptr;
            }
            if (slow == fast) {
                fast = head;
                while (slow != fast) {
                    slow = slow->next;
                    fast = fast->next;
                }
                return slow;
            }
        }
    }
};

int main() {
    auto head = parse_singly_linked_list({3, 2, 0, -4});
    int pos = 1;
    if (pos >= 0) {
        ListNode *h = head;
        while (pos > 0) {
            h = h->next;
            pos--;
        }
        auto first = h;
        while (h->next) {
            h = h->next;
        }
        h->next = first;
        assert(Solution().detectCycle(head) == first);
    } else {
        assert(Solution().detectCycle(head) == nullptr);
    }
}
