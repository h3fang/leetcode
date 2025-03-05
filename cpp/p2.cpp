#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        ListNode dummy = ListNode(0);
        ListNode *h = &dummy;
        int carry = 0;
        while (l1 || l2 || carry) {
            int v = (l1 ? l1->val : 0) + (l2 ? l2->val : 0) + carry;
            carry = v / 10;
            v = v % 10;
            h->next = new ListNode(v);
            h = h->next;
            if (l1) {
                l1 = l1->next;
            }
            if (l2) {
                l2 = l2->next;
            }
        }
        return dummy.next;
    }
};

int main() {
    ListNode *l1 = parse_singly_linked_list({2, 4, 3});
    ListNode *l2 = parse_singly_linked_list({5, 6, 4});
    auto r = Solution().addTwoNumbers(l1, l2);
    ListNode *expected = parse_singly_linked_list({7, 0, 8});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}