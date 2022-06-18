#include "helpers.h"

typedef ListNode Node;

class Solution {
public:
    Node *insert(Node *head, int insertVal) {
        auto node = new Node(insertVal);
        if (!head) {
            node->next = node;
            return node;
        }
        if (head->next == head) {
            head->next = node;
            node->next = head;
            return head;
        }
        auto h = head;
        auto next = head->next;
        while (next != head) {
            if ((h->val <= insertVal && next->val >= insertVal) ||
                (h->val > next->val && (insertVal < next->val || insertVal > h->val))) {
                break;
            }
            h = next;
            next = next->next;
        }
        h->next = node;
        node->next = next;
        return head;
    }
};

int main() {
    auto head = parse_singly_linked_list({3, 4, 1});
    auto tail = head;
    while (tail->next) {
        tail = tail->next;
    }
    tail->next = head;
    auto r = Solution().insert(head, 2);
    auto h = r;
    while (h) {
        printf("%d", h->val);
        h = h->next;
        if (h == r) {
            break;
        }
        printf("->");
    }
    printf("\n");
    return 0;
}