#include <cstdio>
#include <stack>
#include <vector>

#include "helpers.h"

using namespace std;

class Node {
public:
    int val;
    Node *prev;
    Node *next;
    Node *child;
};

class Solution {
public:
    Node *flatten(Node *head) {
        stack<Node *> branches;
        Node *r = head;
        while (true) {
            if (!head) {
                break;
            }
            if (head->child) {
                if (head->next) {
                    branches.push(head->next);
                }
                head->child->prev = head;
                head->next = head->child;
                head->child = nullptr;
                head = head->next;
                continue;
            }
            if (!head->next && !branches.empty()) {
                head->next = branches.top();
                branches.pop();
                head->next->prev = head;
            }
            head = head->next;
        }
        return r;
    }
};

Node *parse_doubly_linked_list(const vector<int> nums, int &i) {
    Node n;
    Node *head = &n;
    for (; i < nums.size(); i++) {
        if (nums[i] == NULL_NODE) {
            break;
        }
        head->next = new Node{nums[i], head, nullptr, nullptr};
        head = head->next;
    }
    n.next->prev = nullptr;
    return n.next;
}

Node *parse_multilevel_doubly_linked_list(const vector<int> nums) {
    int i = 0;
    Node *current_level = parse_doubly_linked_list(nums, i), *r = current_level;
    while (i < nums.size() - 1) {
        i++;
        while (nums[i] == NULL_NODE) {
            i++;
            current_level = current_level->next;
        }
        current_level->child = parse_doubly_linked_list(nums, i);
        current_level = current_level->child;
    }
    return r;
}

int main() {
    vector<int> nums = {1, 2, 3, 4, 5, 6, NULL_NODE, NULL_NODE, NULL_NODE, 7, 8, 9, 10, NULL_NODE, NULL_NODE, 11, 12};
    Node *root = parse_multilevel_doubly_linked_list(nums);
    print_list<Node>(Solution().flatten(root));
    return 0;
}