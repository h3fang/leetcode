#include <algorithm>
#include <vector>
#include <cstdio>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

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

    ListNode list, *head = &list;
    for (auto& e : inputs) {
        head->next = new ListNode(e);
        head = head->next;
    }

    auto h = Solution().oddEvenList(list.next);
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}