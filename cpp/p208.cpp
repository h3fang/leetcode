#include <cstdio>
#include <string>
#include <vector>

using namespace std;

class Trie {
    bool end;
    Trie *next[26];

public:
    Trie() : end(), next() {}

    void insert(string word) {
        Trie *t = this;
        for (auto c : word) {
            const auto i = c - 'a';
            if (!t->next[i]) {
                t->next[i] = new Trie();
            }
            t = t->next[i];
        }
        t->end = true;
    }

    bool search(string word) {
        Trie *t = this;
        for (auto c : word) {
            const auto i = c - 'a';
            if (!t->next[i]) {
                return false;
            }
            t = t->next[i];
        }
        return t->end == true;
    }

    bool startsWith(string prefix) {
        Trie *t = this;
        for (auto c : prefix) {
            const auto i = c - 'a';
            if (!t->next[i]) {
                return false;
            }
            t = t->next[i];
        }
        return true;
    }
};

int main() {
    vector<string> insert = {"", "insert", "apple"};
    vector<string> search = {"", "erqw", "insert", "apple"};
    vector<string> prefix = {"", "app", "appa"};

    auto t = Trie();
    for (auto &e : insert) {
        t.insert(e);
    }

    for (auto &e : search) {
        printf("%s search %d\n", e.data(), t.search(e));
    }

    for (auto &e : prefix) {
        printf("%s startsWith %d\n", e.data(), t.startsWith(e));
    }
    return 0;
}