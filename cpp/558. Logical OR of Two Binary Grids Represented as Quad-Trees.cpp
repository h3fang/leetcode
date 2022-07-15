#include <cassert>
#include <cstdio>
#include <deque>
#include <vector>

using namespace std;

// Definition for a QuadTree node.
class Node {
public:
    bool val;
    bool isLeaf;
    Node *topLeft;
    Node *topRight;
    Node *bottomLeft;
    Node *bottomRight;

    Node() {
        val = false;
        isLeaf = false;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = NULL;
        topRight = NULL;
        bottomLeft = NULL;
        bottomRight = NULL;
    }

    Node(bool _val, bool _isLeaf, Node *_topLeft, Node *_topRight, Node *_bottomLeft,
         Node *_bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }

    static Node *from_grid(const vector<vector<int>> &grid) {
        if (grid.empty()) {
            return nullptr;
        }
        Node *root = new Node(grid[0][1], grid[0][0]);
        deque<Node *> q = {root};
        int i = 1;
        while (!q.empty()) {
            auto n = q.front();
            q.pop_front();
            if (n->isLeaf) {
                i += 4;
                continue;
            }
            n->topLeft = new Node(grid[i][1], grid[i][0]);
            q.push_back(n->topLeft);
            i++;
            n->topRight = new Node(grid[i][1], grid[i][0]);
            q.push_back(n->topRight);
            i++;
            n->bottomLeft = new Node(grid[i][1], grid[i][0]);
            q.push_back(n->bottomLeft);
            i++;
            n->bottomRight = new Node(grid[i][1], grid[i][0]);
            q.push_back(n->bottomRight);
            i++;
            if (i == grid.size()) {
                break;
            }
        }
        return root;
    }
};

class Solution {
public:
    Node *intersect(Node *quadTree1, Node *quadTree2) {
        if (quadTree1->isLeaf) {
            if (quadTree1->val) {
                return new Node(true, true);
            } else {
                return quadTree2;
            }
        }

        if (quadTree2->isLeaf) {
            return intersect(quadTree2, quadTree1);
        }

        Node *n = new Node();
        auto bl = intersect(quadTree1->bottomLeft, quadTree2->bottomLeft);
        auto br = intersect(quadTree1->bottomRight, quadTree2->bottomRight);
        auto tl = intersect(quadTree1->topLeft, quadTree2->topLeft);
        auto tr = intersect(quadTree1->topRight, quadTree2->topRight);
        if (bl->isLeaf && br->isLeaf && tl->isLeaf && tr->isLeaf && br->val == bl->val &&
            br->val == tr->val && br->val == tl->val) {
            n->isLeaf = true;
            n->val = br->val;
        } else {
            n->bottomLeft = bl;
            n->bottomRight = br;
            n->topLeft = tl;
            n->topRight = tr;
        }
        return n;
    }
};

bool is_equal(Node *a, Node *b) {
    if (!a && !b) {
        return true;
    }
    if (!(a && b)) {
        return false;
    }
    if (a->isLeaf && b->isLeaf) {
        return a->val == b->val;
    }
    if (a->isLeaf || b->isLeaf) {
        return false;
    }
    return is_equal(a->topLeft, b->topLeft) && is_equal(a->topRight, b->topRight) &&
           is_equal(a->bottomLeft, b->bottomLeft) && is_equal(a->bottomRight, b->bottomRight);
}

int main() {
    vector<vector<int>> g1 = {{0, 1}, {1, 1}, {1, 1}, {1, 0}, {1, 0}};
    vector<vector<int>> g2 = {{0, 1}, {1, 1}, {0, 1}, {1, 1}, {1, 0}, {},    {},
                              {},     {},     {1, 0}, {1, 0}, {1, 1}, {1, 1}};
    auto q1 = Node::from_grid(g1);
    auto q2 = Node::from_grid(g2);
    auto r = Solution().intersect(q1, q2);
    vector<vector<int>> g3 = {{0, 0}, {1, 1}, {1, 1}, {1, 1}, {1, 0}};
    auto q3 = Node::from_grid(g3);
    assert(is_equal(r, q3));
    return 0;
}