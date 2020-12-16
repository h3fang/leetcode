#include <string>

using std::string;

class Trie {
    Trie *next[128] = {nullptr};
    bool end = false;

public:
    void insert(const string& word) {
        auto t = this;
        for (char w : word) {
            if (!t->next[w]) {
                t->next[w] = new Trie();
            }
            t = t->next[w];
        }
        t->end = true;
    }

    bool search(const string& word) const {
        auto t = this;
        for (char w : word) {
            if (!t->next[w]) {
                return false;
            }
            t = t->next[w];
        }
        return t->end;
    }

    bool startWith(const string& prefix) const {
        auto t = this;
        for (char w : prefix) {
            if (!t->next[w]) {
                return false;
            }
            t = t->next[w];
        }
        return true;
    }
};
