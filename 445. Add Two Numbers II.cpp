#include <cstdio>
#include <vector>
#include "helpers.h"

using namespace std;

class Solution {
public:
    vector<int> list_to_int(ListNode* list) {
        vector<int> r;
        while (list) {
            r.push_back(list->val);
            list = list->next;
        }
        return r;
    }

    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        auto n1 = list_to_int(l1);
        auto n2 = list_to_int(l2);
        auto i1 = n1.rbegin(), i2 = n2.rbegin();
        vector<int> n3;
        int c = 0;
        ListNode *head = nullptr;
        while (i1 != n1.rend() || i2 != n2.rend()) {
            int d = c;
            if (i1 != n1.rend()) {
                d += *i1;
                i1++;
            }
            if (i2 != n2.rend()) {
                d += *i2;
                i2++;
            }
            c = d / 10;
            head = new ListNode(d % 10, head);
        }
        if (c != 0) {
            head = new ListNode(c, head);
        }
        return head;
    }
};

int main() {
    vector<int> list1 = {7,2,4,3}, list2 = {5,6,4};
    auto l1 = parse_singly_linked_list(list1);
    auto l2 = parse_singly_linked_list(list2);
    auto r = Solution().addTwoNumbers(l1, l2);
    print_list(r);
    return 0;
}