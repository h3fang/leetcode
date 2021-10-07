#include <algorithm>
#include <cstdio>
#include <set>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> getSkyline1(vector<vector<int>>& buildings) {
        // use walls to record buildings; left wall is an insertion event, and right wall is a deletion event
        vector<pair<int, int>> walls; // first: x, second: height
        vector<vector<int>> ans;
        for (auto b : buildings) {
            // push in left / right walls
            // let left wall has negative height to ensure left wall goes to multiset first if with same 'x' as right
            // wall
            walls.push_back(make_pair(b[0], -b[2]));
            walls.push_back(make_pair(b[1], b[2]));
        }
        sort(walls.begin(), walls.end()); // sort walls

        multiset<int> leftWallHeights = {0}; // keep left wall heights sorted; dummy '0' for convenience
        int top = 0;                         // current max height among leftWallHeights
        for (auto w : walls) {
            if (w.second < 0) { // it's a left wall, insert the height
                leftWallHeights.insert(-w.second);
            } else { // it's a right wall, delete the height
                leftWallHeights.erase(leftWallHeights.find(w.second));
            }

            if (*leftWallHeights.rbegin() != top) { // mark a skyline point if top changes
                ans.push_back(vector<int>{w.first, top = *leftWallHeights.rbegin()});
            }
        }

        return ans;
    }

    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        vector<pair<int, int>> walls;

        for (const auto& b : buildings) {
            walls.push_back({b[0], -b[2]});
            walls.push_back({b[1], b[2]});
        }

        sort(walls.begin(), walls.end());

        multiset<int> left_walls = {0};
        int highest = 0;
        vector<vector<int>> ans;
        for (const auto& w : walls) {
            if (w.second < 0) {
                left_walls.insert(-w.second);
            }
            else {
                left_walls.erase(left_walls.find(w.second));
            }

            if (const int h = *left_walls.rbegin(); h != highest) {
                highest = h;
                ans.push_back({w.first, highest});
            }
        }

        return ans;
    }
};

int main() {
    vector<vector<int>> buildings = {{2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}};
    auto r = Solution().getSkyline(buildings);
    for (auto &line : r) {
        printf("[%d,%d],", line[0], line[1]);
    }
    printf("\b \n");
    return 0;
}
