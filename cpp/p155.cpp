#include <cassert>
#include <vector>

using namespace std;

class MinStack {
    long m;
    vector<long> q;

public:
    MinStack() {}

    void push(int val) {
        long v = val;
        if (q.empty()) {
            q.push_back(0);
            m = v;
        } else {
            q.push_back(v - m);
            m = min(m, v);
        }
    }

    void pop() {
        long x = q.back();
        q.pop_back();
        if (x < 0) {
            m -= x;
        }
    }

    int top() {
        long x = q.back();
        if (x < 0) {
            return m;
        } else {
            return m + x;
        }
    }

    int getMin() {
        return m;
    }
};

int main() {
    auto s = MinStack();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert(-3 == s.getMin());
    s.pop();
    assert(0 == s.top());
    assert(-2 == s.getMin());

    auto t = MinStack();
    t.push(1);
    t.push(2);
    assert(2 == t.top());
    assert(1 == t.getMin());
    t.pop();
    assert(1 == t.getMin());
    assert(1 == t.top());
    return 0;
}