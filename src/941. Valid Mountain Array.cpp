#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool validMountainArray(vector<int> &arr) {
        int i = 0;
        const int N = arr.size();
        while (i+1 < N && arr[i+1] > arr[i]) {
            i++;
        }
        if (i == 0 || i == N-1) {
            return false;
        }
        while (i+1 < N && arr[i+1] < arr[i]) {
            i++;
        }
        return i == N-1;
    }
};

int main() {
    vector<int> arr = {0, 3, 2, 1};
    printf("%s\n", Solution().validMountainArray(arr) ? "true" : "false");
    return 0;
}
