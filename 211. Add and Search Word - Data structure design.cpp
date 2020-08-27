#include <cstdio>
#include <vector>
#include <string>

using namespace std;

class Trie
{
private:
    Trie* next[26] = {nullptr};
    bool end = false;

public:
    Trie() {};
    ~Trie() {};

    void addWord(const string& w) {
        Trie* t = this;
        for (char c : w) {
            if (!t->next[c - 'a'])  {
                t->next[c - 'a'] = new Trie;
            }
            t = t->next[c - 'a'];
        }
        t->end = true;
    }

    bool searchWord(const string& w) {
        Trie* t = this;
        for (int i = 0; i<w.size(); i++) {
            char c = w[i];
            if (c == '.') {
                for (auto n : t->next) {
                    if (n && n->searchWord(w.substr(i+1))) {
                        return true;
                    }
                }
                return false;
            } else {
                t = t->next[c - 'a'];
                if (!t) {
                    return false;
                }
            }
        }
        return t->end;
    }
};

class WordDictionary {
    Trie t;
public:
    /** Initialize your data structure here. */
    WordDictionary() {
    }

    /** Adds a word into the data structure. */
    void addWord(string word) {
        t.addWord(word);
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    bool search(string word) {
        return t.searchWord(word);
    }
};

int main() {
    vector<string> operations = {"WordDictionary","addWord","addWord","addWord","search","search","search","search"};
    vector<string> operands = {{},{"bad"},{"dad"},{"mad"},{"pad"},{"bad"},{".ad"},{"b.."}};
    WordDictionary* obj = new WordDictionary();

    for (int i = 0; i<operations.size(); i++) {
        if (operations[i] == "addWord") {
            obj->addWord(operands[i]);
        } else if (operations[i] == "search") {
            printf("%s : %d\n", operands[i].data(), obj->search(operands[i]));
        }
    }
    return 0;
}