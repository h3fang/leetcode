#include <cstdio>
#include <stack>
#include <string>
#include <vector>

using namespace std;

// class CombinationIterator {
// public:
//     int index = 0;
//     vector<string> s;

//     CombinationIterator(string characters, int combinationLength) {
//         dfs(0, "", characters, combinationLength);
//     }

//     void dfs(int i, const string &p, const string &characters, const int combinationLength) {
//         if (p.size() == combinationLength) {
//             s.push_back(p);
//             return;
//         }

//         for (int j = i; j < characters.size(); j++) {
//             dfs(j + 1, p + characters[j], characters, combinationLength);
//         }
//     }

//     string next() {
//         return s[index++];
//     }

//     bool hasNext() {
//         return index < s.size();
//     }
// };

class CombinationIterator {
    vector<int> p;
    string characters;
    int combinationLength;

public:
    CombinationIterator(string characters, int combinationLength) {
        this->characters = characters;
        this->combinationLength = combinationLength;
        for (int i = 0; i < combinationLength - 1; i++) {
            p.push_back(i);
        }
        p.push_back(combinationLength - 2);
    }

    string next() {
        string s(combinationLength, 0);
        for (int i = combinationLength - 1; i >= 0; i--) {
            if (p[i] != characters.size() - combinationLength + i) {
                p[i]++;
                for (int j = i + 1; j < combinationLength; j++) {
                    p[j] = p[j - 1] + 1;
                }
                break;
            }
        }
        for (int j = 0; j < combinationLength; j++) {
            s[j] = characters[p[j]];
        }
        return s;
    }

    bool hasNext() {
        return !(p[0] == characters.size() - combinationLength && p[combinationLength - 1] == characters.size() - 1);
    }
};

int main() {
    CombinationIterator *obj = new CombinationIterator("abc", 2);
    vector<string> operations = {"next", "hasNext", "next", "hasNext", "next", "hasNext"};
    for (auto &op : operations) {
        if (op == "next") {
            printf("%s\n", obj->next().data());
        } else if (op == "hasNext") {
            printf("%s\n", obj->hasNext() ? "true" : "false");
        }
    }
    return 0;
}
