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

    ListNode list, *head = &list;
    for (auto& e : inputs) {
        head->next = new ListNode(e);
        head = head->next;
    }

    auto h = Solution().swapPairs(list.next);
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}