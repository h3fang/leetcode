#include <cstdio>
#include <random>
#include <vector>
#include <unordered_map>

using namespace std;

class RandomizedSet {
private:
    vector<int> vals;
    unordered_map<int, int> index;

public:
    /** Initialize your data structure here. */
    RandomizedSet() {
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    bool insert(int val) {
        if (index.find(val) != index.end()) {
            return false;
        }
        vals.push_back(val);
        index[val] = vals.size()-1;
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    bool remove(int val) {
        auto it = index.find(val);
        if (it != index.end()) {
            index[vals[vals.size()-1]] = it->second;
            swap(vals[it->second], vals[vals.size()-1]);
            vals.erase(vals.end()-1);
            index.erase(it);
            return true;
        }
        return false;
    }

    /** Get a random element from the set. */
    int getRandom() {
        int i = rand() % vals.size();
        return vals[i];
    }
};

int main() {
    vector<string> operators = {"RandomizedSet","insert","insert","remove","insert","remove","getRandom"};
    vector<vector<int>> operands = {{},{0},{1},{0},{2},{1},{}};

    RandomizedSet* obj = new RandomizedSet();

    for (int i=0; i < operators.size(); i++) {
        if (operators[i] == "insert") {
            printf("%s ", obj->insert(operands[i][0]) ? "true" : "false");
        } else if (operators[i] == "remove") {
            printf("%s ", obj->remove(operands[i][0]) ? "true" : "false");
        } else if (operators[i] == "getRandom") {
            printf("%d ", obj->getRandom());
        } else if (operators[i] == "RandomizedSet") {
            printf("null ");
        }
    }
    printf("\n");
    return 0;
}