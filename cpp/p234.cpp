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

    bool isPalindrome(ListNode *head) {
        auto fast = head, slow = head;
        while (fast && fast->next) {
            fast = fast->next->next;
            slow = slow->next;
        }
        auto head1 = reverseList(slow);
        while (head1) {
            if (!head || head->val != head1->val) {
                return false;
            }
            head = head->next;
            head1 = head1->next;
        }
        return true;
    }
};

int main() {
    auto head = parse_singly_linked_list({1, 2, 3, 4, 5});
    assert(!Solution().isPalindrome(head));
    head = parse_singly_linked_list({1, 2, 2, 1});
    assert(Solution().isPalindrome(head));
}
