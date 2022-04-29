#include <cstdio>
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

    Node(bool _val, bool _isLeaf, Node *_topLeft, Node *_topRight, Node *_bottomLeft, Node *_bottomRight) {
        val = _val;
        isLeaf = _isLeaf;
        topLeft = _topLeft;
        topRight = _topRight;
        bottomLeft = _bottomLeft;
        bottomRight = _bottomRight;
    }
};

class Solution {
public:
    Node *construct(vector<vector<int>> &grid) {
        return dfs(grid, 0, grid.size(), 0, grid[0].size());
    }

    Node *dfs(vector<vector<int>> &grid, int i1, int i2, int j1, int j2) {
        const bool val = grid[i1][j1];
        bool same_value = true;
        for (int i = i1; i < i2; i++) {
            for (int j = j1; j < j2; j++) {
                if (grid[i][j] != val) {
                    same_value = false;
                    goto the_end;
                }
            }
        }
    the_end:
        if (same_value) {
            auto n = new Node(val, true);
            return n;
        } else {
            auto n = new Node(val, false);
            int i_mid = i1 + (i2 - i1) / 2;
            int j_mid = j1 + (j2 - j1) / 2;
            n->topLeft = dfs(grid, i1, i_mid, j1, j_mid);
            n->topRight = dfs(grid, i1, i_mid, j_mid, j2);
            n->bottomLeft = dfs(grid, i_mid, i2, j1, j_mid);
            n->bottomRight = dfs(grid, i_mid, i2, j_mid, j2);
            return n;
        }
    }
};

int main() {
    vector<vector<int>> grid = {{1, 1, 0, 0}, {0, 0, 1, 1}, {1, 1, 0, 0}, {0, 0, 1, 1}};
    auto r = Solution().construct(grid);
    return 0;
}