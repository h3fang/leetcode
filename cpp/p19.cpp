#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode *removeNthFromEnd(ListNode *head, int n) {
        ListNode dummy = ListNode(0, head);
        ListNode *fast = &dummy, *slow = &dummy;
        for (int i = 0; i <= n; i++) {
            fast = fast->next;
        }
        while (fast) {
            fast = fast->next;
            slow = slow->next;
        }
        slow->next = slow->next->next;
        return dummy.next;
    }
};

int main() {
    ListNode *l1 = parse_singly_linked_list({1, 2, 3, 4, 5});
    auto r = Solution().removeNthFromEnd(l1, 2);
    ListNode *expected = parse_singly_linked_list({1, 2, 3, 5});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}