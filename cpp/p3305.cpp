#include <cassert>
#include <string>
#include <vector>

using namespace std;

int vowel_index(char c) {
    if (c == 'a') {
        return 0;
    } else if (c == 'e') {
        return 1;
    } else if (c == 'i') {
        return 2;
    } else if (c == 'o') {
        return 3;
    } else if (c == 'u') {
        return 4;
    } else {
        return -1;
    }
}

bool has_every_vowel(vector<int> &vowels) {
    for (int c : vowels) {
        if (c == 0) {
            return false;
        }
    }
    return true;
}

long long helper(string &word, int k) {
    long long ans = 0;
    const int n = word.size();
    vector<int> vowels(5, 0);
    int l = 0, r = 0, consonants = 0;
    while (r < n) {
        int i = vowel_index(word[r]);
        if (i >= 0) {
            vowels[i] += 1;
        } else {
            consonants += 1;
        }
        r += 1;

        while (l < r && has_every_vowel(vowels) && consonants >= k) {
            int i = vowel_index(word[l]);
            if (i >= 0) {
                vowels[i] -= 1;
            } else {
                consonants -= 1;
            }
            l += 1;
        }
        ans += l;
    }
    return ans;
}

class Solution {
public:
    int countOfSubstrings(string word, int k) {
        return helper(word, k) - helper(word, k + 1);
    }
};

int main() {
    auto r = Solution().countOfSubstrings("aeioqq", 1);
    assert(r == 0);

    r = Solution().countOfSubstrings("aeiou", 0);
    assert(r == 1);

    r = Solution().countOfSubstrings("ieaouqqieaouqq", 1);
    assert(r == 3);
    return 0;
}