#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <set>
#include <algorithm>
#include <cstdio>

using namespace std;

vector<vector<int>> threeSum(vector<int>& nums) {
    vector<vector<int>> r;

    sort(nums.begin(), nums.end());

    for (int i=0; i< nums.size(); i++) {
        int f = i+1, b = nums.size()-1, t = -nums[i];
        if (t < 0) {continue;}
        while (f < b) {
            int s = nums[f] + nums[b];
            if (s < t) {
                f++;
            }
            else if (s > t) {
                b--;
            }
            else {
                vector<int> e(3,0);
                e[0] = -t;
                e[1] = nums[f];
                e[2] = nums[b];
                r.push_back(e);

                while (f < b && nums[f] == e[1]) {f++;}
                while (f < b && nums[b] == e[2]) {b--;}
            }
        }

        while (i+1 < nums.size() && nums[i+1] == nums[i]) {i++;}
    }

    return r;
}

int main() {
    vector<int> inputs = {-1, 0, 1, 2, -1, -4};

    auto r = threeSum(inputs);
    for (auto& e : r) {
        printf("%d %d %d\n", e[0], e[1], e[2]);
    }
    return 0;
}