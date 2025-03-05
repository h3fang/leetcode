#include <cassert>
#include <unordered_map>

using namespace std;

struct Node {
    int key;
    int val;
    Node *prev;
    Node *next;
    Node() {}
    Node(int key, int val) : key(key), val(val) {}
};

class LRUCache {
    int capacity;
    Node dummy;
    unordered_map<int, Node *> nodes;

    void append_to_tail(Node *n) {
        auto tail = dummy.prev;
        tail->next = n;
        n->prev = tail;
        n->next = &dummy;
        dummy.prev = n;
    }

    Node *get_node(int key) {
        if (nodes.count(key) == 0) {
            return nullptr;
        }
        Node *n = nodes[key];
        n->prev->next = n->next;
        n->next->prev = n->prev;
        append_to_tail(n);
        return n;
    }

public:
    LRUCache(int capacity) : capacity(capacity) {
        dummy.prev = &dummy;
    }

    int get(int key) {
        auto n = get_node(key);
        return n ? n->val : -1;
    }

    void put(int key, int value) {
        auto n = get_node(key);
        if (n) {
            n->val = value;
        } else {
            if (nodes.size() == capacity) {
                n = dummy.next;
                dummy.next = n->next;
                n->next->prev = &dummy;
                nodes.erase(n->key);
                delete n;
            }
            n = new Node(key, value);
            nodes.insert({key, n});
            append_to_tail(n);
        }
    }
};

int main() {
    auto cache = LRUCache(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert(1 == cache.get(1));
    cache.put(3, 3);
    assert(-1 == cache.get(2));
    cache.put(4, 4);
    assert(-1 == cache.get(1));
    assert(3 == cache.get(3));
    assert(4 == cache.get(4));
    return 0;
}