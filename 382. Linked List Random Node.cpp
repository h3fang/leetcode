#include <cstdio>
#include <vector>
#include <random>
#include <map>

#include "helpers.h"

using namespace std;

class Solution {
    ListNode *head;
    default_random_engine gen;

public:
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    Solution(ListNode *head) {
        this->head = head;
    }

    /** Returns a random node's value. */
    int getRandom() {
        auto h = this->head;
        if (!h) {return 0;}
        int val = h->val;
        int i = 1;
        h = h->next;
        while (h) {
            i++;
            int j = uniform_int_distribution(1, i)(gen);
            if (j <= 1) {
                val = h->val;
            }
            h = h->next;
        }
        return val;
    }
};

int main() {
    vector<int> nodes = {1, 2, 3};
    auto sll = parse_singly_linked_list(nodes);
    Solution s(sll);
    map<int, int> count;
    const int total_trials = 1000000;
    for (int i = 0; i < total_trials; i++) {
        count[s.getRandom()]++;
    }
    for (auto [k,v] : count) {
        printf("%d: %.5f\n", k, double(v)/total_trials);
    }
    return 0;
}
