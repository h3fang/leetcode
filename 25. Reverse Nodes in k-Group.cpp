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

    ListNode list, *head = &list;
    for (auto& e : inputs) {
        head->next = new ListNode(e);
        head = head->next;
    }

    auto h = Solution().reverseKGroup(list.next, 3);
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}