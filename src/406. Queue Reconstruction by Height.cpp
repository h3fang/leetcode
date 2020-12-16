#include <cstdio>
#include <algorithm>
#include <vector>
#include <deque>

using namespace std;

class Solution {
public:
    vector<vector<int>> reconstructQueue2(vector<vector<int>>& people) {
        sort(people.begin(), people.end(), [](const auto & l, const auto & r){
            if (l[1] == r[1]) {
                return l[0] < r[0];
            } else {
                return l[1] < r[1];
            }
        });

        deque<vector<int>> q(people.begin(), people.end());

        for (int i=1; i < people.size(); i++) {
            const auto p = people[i];
            if (p[1] == 0) {continue;}
            int c = 0;
            for (int j = 0; j<i; j++) {
                if (q[j][0] >= p[0]) {
                    c++;
                }
                if (c > p[1]) {
                    q.erase(q.begin()+i);
                    q.insert(q.begin()+j, p);
                    break;
                }
            }
        }

        return vector<vector<int>>(q.begin(), q.end());
    }

    vector<vector<int>> reconstructQueue(vector<vector<int>>& people) {
        sort(people.begin(), people.end(), [](const auto & l, const auto & r){
            if (l[0] == r[0]) { return l[1] < r[1]; }
            else { return l[0] > r[0]; }
        });

        vector<vector<int>> r;
        for (const auto& p : people) {
            r.insert(r.begin() + p[1], p);
        }
        return r;
    }
};

int main() {
    vector<vector<int>> people = {{9,0},{7,0},{1,9},{3,0},{2,7},{5,3},{6,0},{3,4},{6,2},{5,2}};
    for (auto & p : Solution().reconstructQueue(people)) {
        printf("[%d,%d] ", p[0], p[1]);
    }
    printf("\n");
    return 0;
}