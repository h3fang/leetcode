#include <cstdio>
#include <vector>
#include <set>
#include <string>

using namespace std;

class MyHashSet {
    vector<set<int>> storage;

    int hash(long key) {
        return ((key*1031237) & (1<<20) - 1)>>5;
    }

public:
    /** Initialize your data structure here. */
    MyHashSet() {
        storage.resize(1<<15);
    }

    void add(int key) {
        int h = hash(key);
        if (storage[h].count(key) == 0) {
            storage[h].insert(key);
        }
    }

    void remove(int key) {
        int h = hash(key);
        if (storage[h].count(key) == 1) {
            storage[h].erase(key);
        }
    }

    /** Returns true if this set contains the specified element */
    bool contains(int key) {
        int h = hash(key);
        return storage[h].count(key) == 1;
    }
};

int main() {
    vector<string> operations = {"MyHashSet","add","add","contains","contains","add","contains","remove","contains"};
    vector<vector<int>> operands = {{},{1},{2},{1},{3},{2},{2},{2},{2}};
    MyHashSet* obj = new MyHashSet();
    for (int i = 0; i < operations.size(); i++) {
        if (operations[i] == "add") {
            obj->add(operands[i][0]);
        } else if (operations[i] == "remove") {
            obj->remove(operands[i][0]);
        } else if (operations[i] == "contains") {
            printf("contains %d : %d\n", operands[i][0], obj->contains(operands[i][0]));
        }
    }
    delete obj;
    return 0;
}