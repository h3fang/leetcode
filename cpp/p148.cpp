#include <algorithm>
#include <cassert>
#include <vector>

#include "helpers.h"

using namespace std;

class Solution {
    ListNode *split(ListNode *head, int size) {
        for (int i = 1; i < size && head; i++) {
            head = head->next;
        }
        if (!head || !head->next) {
            return nullptr;
        } else {
            auto tail = head->next;
            head->next = nullptr;
            return tail;
        }
    }
    pair<ListNode *, ListNode *> merge(ListNode *l1, ListNode *l2) {
        ListNode d(0), *h = &d;
        while (l1 && l2) {
            if (l1->val <= l2->val) {
                h->next = l1;
                l1 = l1->next;
            } else {
                h->next = l2;
                l2 = l2->next;
            }
            h = h->next;
        }
        h->next = l1 ? l1 : l2;
        while (h->next) {
            h = h->next;
        }
        return {d.next, h};
    }

public:
    ListNode *sortList(ListNode *head) {
        int n = 0;
        ListNode *h = head;
        while (h) {
            n += 1;
            h = h->next;
        }
        ListNode d(0, head);
        for (int i = 1; i < n; i *= 2) {
            ListNode *prev = &d;
            h = d.next;
            while (h) {
                ListNode *l1 = h;
                ListNode *l2 = split(l1, i);
                h = split(l2, i);
                auto [head, tail] = merge(l1, l2);
                prev->next = head;
                prev = tail;
            }
        }
        return d.next;
    }
};

int main() {
    auto r = Solution().sortList(parse_singly_linked_list({4, 2, 1, 3}));
    ListNode *expected = parse_singly_linked_list({1, 2, 3, 4});
    while (expected) {
        assert(r && r->val == expected->val);
        expected = expected->next;
        r = r->next;
    }
    assert(!r);
    return 0;
}