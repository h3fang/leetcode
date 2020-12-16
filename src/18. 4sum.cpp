#include <vector>
#include <map>
#include <algorithm>
#include <cstdio>

using namespace std;

class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        const int N = nums.size();
        vector<vector<int>> r;

        if (N < 4) {return r;}

        sort(nums.begin(), nums.end());

        int f,b,t;

        for (int i=0; i<N-3; i++) {
            if (target < nums[i]+nums[i+1]+ nums[i+2]+nums[i+3]) {break;}
            for (int j=i+1; j<N-2; j++) {
                if (target < nums[i]+nums[j]+ nums[j+1]+nums[j+2]) {break;}
                t = target - nums[i] - nums[j];
                f = j+1; b = N-1;
                while(f < b) {
                    int c = nums[f] + nums[b];
                    if (c < t) {
                        f++;
                    }
                    else if (c > t) {
                        b--;
                    }
                    else {
                        r.push_back(vector<int>{nums[i], nums[j], nums[f], nums[b]});
                        f++; b--;
                        while (f<b && nums[f]==nums[f-1]) {f++;}
                        while (f<b && nums[b]==nums[b+1]) {b--;}
                    }
                }
                while(j+1<N-2 && nums[j+1]==nums[j]) {j++;}
            }
            while(i+1<N-3 && nums[i+1]==nums[i]) {i++;}
        }
        return r;
    }
};

int main() {
    vector<int> nums = {-9,4,0,-3,6,3,-3,-9,-7,1,0,-7,-8,7,1};
    int target = -9;

    auto s = Solution().fourSum(nums, target);

    for (auto& e : s) {
        printf("%d %d %d %d\n", e[0], e[1], e[2], e[3]);
    }

    return 0;
}