#include <cstdio>
#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

class Solution {
    const int base = 26;
    const int modulus = 19260817;
    vector<int64_t> offsets;

public:
    int hash(const string& s) {
        int64_t r = 0;
        for (char c : s) {
            r = ((r * base) + (c - 'a')) % modulus;
        }
        return r;
    }

    int hash(const int64_t prev, const int64_t next, const int64_t h_prev, const int64_t offset) {
        return ((((h_prev - (prev - 'a') * offset) * base) % modulus + modulus) % modulus + (next - 'a')) % modulus;
    }

    string rk_search(const string& s, int size) {
        int offset = offsets[size-1], h_prev = hash(s.substr(0, size));
        unordered_map<int, vector<int>> hashes;
        hashes[h_prev] = {0};
        for (int i=1; i<=s.size()-size; i++) {
            int h = hash(s[i-1], s[i+size-1], h_prev, offset);
            h_prev = h;
            if (hashes.find(h) == hashes.end()) {
                hashes[h] = vector<int>{i};
            } else {
                for (int p : hashes[h]) {
                    bool same = true;
                    for (int k = 0; k < size; k++) {
                        if (s[i+k] != s[p+k]) {
                            same = false;
                            break;
                        }
                    }
                    if (same) {
                        return s.substr(p, size);
                    }
                }
                hashes[h].push_back(i);
            }
        }
        return "";
    }

    string longestDupSubstring(string S) {
        int a = 1, b = S.size()-1;
        offsets.resize(b+1, 1);
        for (int i=1; i<=b; i++) {
            offsets[i] = (offsets[i-1] * base) % modulus;
        }
        string r;
        while (a<=b) {
            int c = a + (b-a)/2;
            string match = rk_search(S, c);
            if (match.size()) {
                r = match;
                a = c + 1;
            } else {
                b = c - 1;
            }
        }
        return r;
    }
};

int main() {
    string str = "okmzpmxzwjbfssktjtebhhxfphcxefhonkncnrumgduoaeltjvwqwydpdsrbxsgmcdxrthilniqxkqzuuqzqhlccmqcmccfqddncchadnthtxjruvwsmazlzhijygmtabbzelslebyrfpyyvcwnaiqkkzlyillxmkfggyfwgzhhvyzfvnltjfxskdarvugagmnrzomkhldgqtqnghsddgrjmuhpgkfcjkkkaywkzsikptkrvbnvuyamegwempuwfpaypmuhhpuqrufsgpiojhblbihbrpwxdxzolgqmzoyeblpvvrnbnsdnonhpmbrqissifpdavvscezqzclvukfgmrmbmmwvzfpxcgecyxneipexrzqgfwzdqeeqrugeiupukpveufmnceetilfsqjprcygitjefwgcvqlsxrasvxkifeasofcdvhvrpmxvjevupqtgqfgkqjmhtkyfsjkrdczmnettzdxcqexenpxbsharuapjmdvmfygeytyqfcqigrovhzbxqxidjzxfbrlpjxibtbndgubwgihdzwoywqxegvxvdgaoarlauurxpwmxqjkidwmfuuhcqtljsvruinflvkyiiuwiiveplnxlviszwkjrvyxijqrulchzkerbdyrdhecyhscuojbecgokythwwdulgnfwvdptzdvgamoublzxdxsogqpunbtoixfnkgbdrgknvcydmphuaxqpsofmylyijpzhbqsxryqusjnqfikvoikwthrmdwrwqzrdmlugfglmlngjhpspvnfddqsvrajvielokmzpmxzwjbfssktjtebhhxfphcxefhonkncnrumgduoaeltjvwqwydpdsrbxsgmcdxrthilniqxkqzuuqzqhlccmqcmccfqddncchadnthtxjruvwsmazlzhijygmtabbzelslebyrfpyyvcwnaiqkkzlyillxmkfggyfwgzhhvyzfvnltjfxskdarvugagmnrzomkhldgqtqnghsddgrjmuhpgkfcjkkkaywkzsikptkrvbnvuyamegwempuwfpaypmuhhpuqrufsgpiojhblbihbrpwxdxzolgqmzoyeblpvvrnbnsdnonhpmbrqissifpdavvscezqzclvukfgmrmbmmwvzfpxcgecyxneipexrzqgfwzdqeeqrugeiupukpveufmnceetilfsqjprcygitjefwgcvqlsxrasvxkifeasofcdvhvrpmxvjevupqtgqfgkqjmhtkyfsjkrdczmnettzdxcqexenpxbsharuapjmdvmfygeytyqfcqigrovhzbxqxidjzxfbrlpjxibtbndgubwgihdzwoywqxegvxvdgaoarlauurxpwmxqjkidwmfuuhcqtljsvruinflvkyiiuwiiveplnxlviszwkjrvyxijqrulchzkerbdyrdhecyhscuojbecgokythwwdulgnfwvdptzdvgamoublzxdxsogqpunbtoixfnkgbdrgknvcydmphuaxqpsofmylyijpzhbqsxryqusjnqfikvoikwthrmdwrwqzrdmlugfglmlngjhpspvnfddqsvrajviel";
    printf("%s\n", Solution().longestDupSubstring(str).data());
    return 0;
}