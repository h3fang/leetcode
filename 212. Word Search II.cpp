#include <cstdio>
#include <vector>
#include <string>

using namespace std;

class Trie {
public:
    Trie() {
    }

    void insert(string word) {
        Trie * t = this;
        for (auto c : word) {
            const auto i = c-'a';
            if (!t->next[i]) {
                t->next[i] = new Trie();
            }
            t = t->next[i];
        }
        t->end = true;
    }

    bool isEmpty() {
        for (int i = 0; i < 26; i++) {
            if (next[i]) {
                return false;
            }
        }
        return true;
    }

public:
    Trie *next[26] = {nullptr};
    bool end = false;
};

class Solution {
    Trie trie;
    vector<string> r;

public:
    void bt(vector<vector<char>>& board, int i, int j, string& s, Trie* t) {
        if (i < 0 || j < 0 || i >= board.size() || j >= board[0].size() || board[i][j] == 0) {
            return;
        }

        char c = board[i][j];

        auto n = t->next[c-'a'];

        if (!n) {
            return;
        }

        if (n->end) {
            r.push_back(s+c);
            n->end = false;
        }

        s += c;

        board[i][j] = 0;

        bt(board, i-1, j, s, n);
        bt(board, i+1, j, s, n);
        bt(board, i, j-1, s, n);
        bt(board, i, j+1, s, n);

        s.pop_back();
        board[i][j] = c;

        if (n->isEmpty()) {
            delete n;
            t->next[c - 'a'] = nullptr;
        }

        return;
    }

    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        for (auto& w : words) {
            trie.insert(w);
        }
        r.clear();

        for (int i=0; i<board.size(); i++) {
            for (int j=0; j<board[0].size(); j++) {
                string s;
                bt(board, i, j, s, &trie);
            }
        }
        return r;
    }
};

int main() {
    vector<vector<char>> board = {{'o','a','a','n'},{'e','t','a','e'},{'i','h','k','r'},{'i','f','l','v'}};
    vector<string> words = {"oath","pea","eat","rain"};
    // vector<vector<char>> board = {{'a', 'a'}};
    // vector<string> words = {"a"};
    for (auto& s : Solution().findWords(board, words)) {
        printf("%s ", s.data());
    }
    printf("\n");
    return 0;
}