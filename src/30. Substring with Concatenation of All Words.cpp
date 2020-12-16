#include <vector>
#include <unordered_map>
#include <string>
#include <cstdio>

using namespace std;

class Solution {
public:
    int mismatch(unordered_map<string, int>& t) {
        int c = 0;
        for (const auto& [k,v] : t) {
            if (v != 0) {
                c++;
            }
        }
        return c;
    }
    vector<int> findSubstring(string s, vector<string>& words) {
        const int N = s.size(), K = words.size();
        vector<int> r;
        if (N == 0 || K == 0) {return r;}
        const int M = words[0].size();
        if (M == 0 || N < M*K) {return r;}
        unordered_map<string, int> t0;
        for (const auto& w: words) {
            t0[w]++;
        }
        for (int m=0; m<M; m++) {
            if (N-m < M*K) {break;}
            unordered_map<string, int> t = t0;
            for (int k=0; k<K; k++) {
                t[s.substr(m+k*M, M)]--;
            }
            int delta = mismatch(t);
            if (delta == 0) {r.push_back(m);}
            string w;
            for (int i=m+M; i<N; i+=M) {
                if (N - i < M*K) {break;}
                w = s.substr(i-M, M);
                int c = t[w];
                if (c == -1) {delta--;}
                else if (c == 0) {delta++;}
                t[w]++;
                w = s.substr(i+(K-1)*M, M);
                c = t[w];
                if (c == 1) {delta--;}
                else if (c == 0) {delta++;}
                t[w]--;
                if (delta == 0) {r.push_back(i);}
            }
        }
        return r;
    }
};

int main() {
    string s = "abbbbbacbc";
    vector<string> words = {"bc","ac"};

    for (auto i : Solution().findSubstring(s, words)) {
        printf("%d ", i);
    }
    printf("\n");
    return 0;
}