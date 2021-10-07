#include <cstdio>
#include <vector>

using namespace std;

class Solution {
public:
    bool canPlaceFlowers(vector<int>& flowerbed, int n) {
		int spare = 0;
		for (int b : flowerbed) {
			if (b == 0) {
				spare += 1;
			}
		}
		if (n * 2 - 1 > spare) {
			return false;
		}

		const int N = flowerbed.size();

		for (int i = 0; i < N; i++) {
			if (flowerbed[i] ==0 && (i == 0 || (flowerbed[i-1] == 0)) && (i+1 == N || (flowerbed[i+1] == 0))) {
				flowerbed[i] = 1;
				i++;
				if (--n <= 0) {
					return true;
				}
			}
		}

		return n <= 0;
    }
};


int main() {
    vector<int> flowerbed = {1,0,0,0,1};
	const int n = 2;
    printf("%s\n", Solution().canPlaceFlowers(flowerbed, n) ? "true" : "false");
    return 0;
}