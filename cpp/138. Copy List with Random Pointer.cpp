#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

#define null -1000000

class Node {
public:
    int val;
    Node *next;
    Node *random;

    Node(int _val) {
        val = _val;
        next = nullptr;
        random = nullptr;
    }
};

class Solution {
    unordered_map<Node *, Node *> map;

public:
    Node *copyRandomList_hashmap(Node *head) {
        if (!head) {
            return head;
        }
        auto copy = new Node(head->val);
        map[head] = copy;
        copy->next = copyRandomList_hashmap(head->next);
        copy->random = head->random ? map[head->random] : nullptr;
        return copy;
    }

    Node *copyRandomList(Node *head) {
        if (!head) {
            return head;
        }
        for (auto h = head; h; h = h->next->next) {
            auto copy = new Node(h->val);
            copy->next = h->next;
            h->next = copy;
        }
        for (auto h = head; h; h = h->next->next) {
            auto copy = h->next;
            copy->random = h->random ? h->random->next : nullptr;
        }
        auto result = head->next;
        for (auto h = head; h; h = h->next) {
            auto copy = h->next;
            h->next = copy->next;
            if (h->next) {
                copy->next = h->next->next;
            }
        }
        return result;
    }
};

int main() {
    unordered_map<int, Node *> map;
    vector<vector<int>> edges = {{7, null}, {13, 0}, {11, 4}, {10, 2}, {1, 0}};
    for (int i = 0; i < edges.size(); i++) {
        map[edges[i][0]] = new Node(edges[i][0]);
        if (i > 0) {
            map[edges[i - 1][0]]->next = map[edges[i][0]];
        }
    }
    for (auto &e : edges) {
        if (e[1] != null) {
            map[e[0]]->random = map[e[1]];
        }
    }
    auto head = map[edges[0][0]];
    Solution s;
    auto copy = s.copyRandomList(head);
    while (head) {
        assert(copy != nullptr && copy != head);
        assert(copy->val == head->val);
        if (head->random) {
            assert(copy->random != nullptr && copy->random != head->random);
            assert(copy->random->val == head->random->val);
        } else {
            assert(!copy->random);
        }
        head = head->next;
        copy = copy->next;
    }
    assert(!copy);
    return 0;
}