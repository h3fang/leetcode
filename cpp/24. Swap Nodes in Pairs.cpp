#include <algorithm>
#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode *swapPairs(ListNode *head) {
        ListNode r(0, head), *prev = &r;
        while (head && head->next) {
            auto a = head, b = head->next, c = head->next->next;
            prev->next = b;
            b->next = a;
            a->next = c;
            head = c;
            prev = a;
        }
        return r.next;
    }
};

int main() {
    auto r = Solution().swapPairs(parse_singly_linked_list({1, 2, 3, 4}));
    ListNode *expected = parse_singly_linked_list({2, 1, 4, 3});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}