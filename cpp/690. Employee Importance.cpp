#include <cassert>
#include <cstdio>
#include <unordered_map>
#include <vector>

#include "helpers.h"

using namespace std;

class Employee {
public:
    int id;
    int importance;
    vector<int> subordinates;
};

class Solution {
    unordered_map<int, Employee *> g;

public:
    int getImportance(vector<Employee *> employees, int id) {
        int n = employees.size();
        g.reserve(n);
        for (auto e : employees) {
            g[e->id] = e;
        }
        return dfs(id);
    }

    int dfs(int x) {
        int result = g[x]->importance;
        for (auto y : g[x]->subordinates) {
            result += dfs(y);
        }
        return result;
    }
};

void test(vector<Employee> emps, int id, int exptected) {
    vector<Employee *> employees;
    for (auto &&e : emps) {
        employees.push_back(&e);
    }
    int result = Solution().getImportance(employees, id);
    assert(result == exptected);
}

int main() {
    vector<Employee> emps1 = {{1, 5, {2, 3}}, {2, 3, {}}, {3, 3, {}}};
    test(emps1, 1, 11);

    vector<Employee> emps2 = {{1, 2, {5}}, {5, -3, {}}};
    test(emps2, 5, -3);

    return 0;
}