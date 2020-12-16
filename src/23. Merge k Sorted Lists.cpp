#include <algorithm>
#include <vector>
#include <queue>
#include <cstdio>

#include "helpers.h"

using namespace std;

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        auto cmp = [](ListNode* l, ListNode* r){ return l->val > r->val;};
        priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> q(cmp);

        for (auto l : lists) {
            if (l) {
                q.push(l);
            }
        }

        ListNode *d = new ListNode, *r = d;

        while (!q.empty()) {
            auto l = q.top();
            q.pop();
            if (l->next) {
                q.push(l->next);
            }
            r->next = l;
            r = l;
        }

        r = d->next;
        delete d;
        return r;
    }
};

int main() {
    vector<vector<int>> inputs = {{1,4,5},{1,3,4},{2,6}};

    vector<ListNode*> lists;
    ListNode list, *head = nullptr;
    for (auto& l : inputs) {
        lists.push_back(parse_singly_linked_list(l));
    }

    auto h = Solution().mergeKLists(lists);
    while (h) {
        printf("%d ", h->val);
        h = h->next;
    }
    printf("\n");
    return 0;
}