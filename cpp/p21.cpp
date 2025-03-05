#include <algorithm>
#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode *mergeTwoLists(ListNode *list1, ListNode *list2) {
        ListNode dummy = ListNode(0);
        ListNode *h = &dummy;
        while (list1 || list2) {
            if (list1 && (!list2 || list1->val < list2->val)) {
                h->next = list1;
                list1 = list1->next;
            } else {
                h->next = list2;
                list2 = list2->next;
            }
            h = h->next;
        }
        return dummy.next;
    }
};

int main() {
    ListNode *l1 = parse_singly_linked_list({1, 2, 4});
    ListNode *l2 = parse_singly_linked_list({1, 3, 4});
    auto r = Solution().mergeTwoLists(l1, l2);
    ListNode *expected = parse_singly_linked_list({1, 1, 2, 3, 4, 4});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}