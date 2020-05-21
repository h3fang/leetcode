#include <algorithm>
#include <vector>
#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    void reverse(ListNode* h, ListNode* t) {
        ListNode *p = nullptr;
        while (h != t) {
            auto a = h->next;
            h->next = p;
            p = h;
            h = a;
        }
        t->next = p;
    }
    ListNode* reverseKGroup(ListNode* head, int k) {
        if (k == 1) {return head;}
        ListNode d, *r = &d;
        ListNode* t,*h = head;
        const int m = k;
        while (head) {
            if (m == k) {
                h = head;
            }
            k--;
            if (k == 0) {
                t = head;
                head = head->next;
                reverse(h, t);
                r->next = t;
                r = h;
                k = m;
                h = head;
            }
            else {
                head = head->next;
            }
        }
        if (k < m) r->next = h;
        else r->next = nullptr;
        return d.next;
    }
};

int main() {
    vector<int> inputs = {1,2,3,4,5};

    auto h = Solution().reverseKGroup(parse_singly_linked_list(inputs), 3);
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}