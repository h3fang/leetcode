#include <algorithm>
#include <vector>
#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode* swapPairs(ListNode* head) {
        ListNode r, *h=&r;
        h->next = head;
        while (head) {
            auto t = head;
            if (head->next) {
                h->next = head->next;
                head = head->next->next;
                h->next->next = t;
                h = t;
            }
            else {
                h->next = head;
                h = head;
                head = head->next;
            }
        }
        h->next = nullptr;
        return r.next;
    }
};

int main() {
    vector<int> inputs = {1,2,3,4,5};

    auto h = Solution().swapPairs(parse_singly_linked_list(inputs));
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}