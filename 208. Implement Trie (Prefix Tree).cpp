#include <algorithm>
#include <vector>
#include <string>
#include <map>
#include <set>
#include <cstdio>

using namespace std;

class Trie {
public:
    /** Initialize your data structure here. */
    Trie() {
    }

    /** Inserts a word into the trie. */
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

    /** Returns if the word is in the trie. */
    bool search(string word) {
        Trie * t = this;
        for (auto c : word) {
            const auto i = c-'a';
            if (t->next[i]) {
                t = t->next[i];
            }
            else {
                return false;
            }
        }
        return t->end == true;
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    bool startsWith(string prefix) {
        Trie * t = this;
        for (auto c : prefix) {
            const auto i = c-'a';
            if (t->next[i]) {
                t = t->next[i];
            }
            else {
                return false;
            }
        }
        return true;
    }

private:
    Trie *next[26] = {nullptr};
    bool end = false;
};

int main() {
    vector<string> insert = {"", "insert", "apple"};
    vector<string> search = {"", "erqw", "insert", "apple"};
    vector<string> prefix = {"", "app", "appa"};

    auto t = Trie();
    for (auto& e : insert) {
        t.insert(e);
    }

    for (auto& e : search) {
        printf("%s search %d\n", e.data(), t.search(e));
    }

    for (auto& e : prefix) {
        printf("%s startsWith %d\n", e.data(), t.startsWith(e));
    }
    return 0;
}