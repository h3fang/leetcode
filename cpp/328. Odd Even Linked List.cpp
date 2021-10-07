#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode* oddEvenList(ListNode* head) {
        ListNode l1, l2, *o = &l1, *e = &l2;
        while(head) {
            o->next = head;
            o = head;
            head = head->next;
            if (head) {
                e->next = head;
                e = head;
                head = head->next;
            }
        }
        if (o) o->next = l2.next;
        if (e) e->next = nullptr;
        return l1.next;
    }
};

int main() {
    vector<int> inputs = {1,2,3,4,5};

    auto h = Solution().oddEvenList(parse_singly_linked_list(inputs));
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}