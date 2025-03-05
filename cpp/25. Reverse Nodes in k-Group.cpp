#include <algorithm>
#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
public:
    void reverse(ListNode *h, ListNode *t) {
        ListNode *p = nullptr;
        while (h != t) {
            auto a = h->next;
            h->next = p;
            p = h;
            h = a;
        }
        t->next = p;
    }
    ListNode *reverseKGroup(ListNode *head, int k) {
        if (k == 1) {
            return head;
        }
        ListNode d(0, head), *prev = &d;
        while (head) {
            auto tail = head;
            for (int i = 1; i < k; i++) {
                tail = tail->next;
                if (!tail) {
                    return d.next;
                }
            }
            auto next = tail->next;
            reverse(head, tail);
            prev->next = tail;
            head->next = next;
            prev = head;
            head = next;
        }
        return d.next;
    }
};

int main() {
    auto r = Solution().reverseKGroup(parse_singly_linked_list({1, 2, 3, 4, 5}), 2);
    ListNode *expected = parse_singly_linked_list({2, 1, 4, 3, 5});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}