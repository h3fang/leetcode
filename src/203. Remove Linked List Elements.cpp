#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode* removeElements(ListNode* head, int val) {
        ListNode* prev = nullptr, * r = nullptr;
        while(head) {
            if (head->val == val) {
                head = head->next;
                if (prev) {
                    prev->next = head;
                }
            } else {
                if (!prev) {
                    r = head;
                }
                prev = head;
                head = head->next;
            }
        }
        return r;
    }
};

int main() {
    vector<int> nodes = {1,2,6,3,4,5,6};
    auto head = parse_singly_linked_list(nodes);
    print_list(Solution().removeElements(head, 6));
    return 0;
}
