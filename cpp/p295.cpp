#include <algorithm>
#include <cassert>
#include <queue>
#include <vector>

using namespace std;

class MedianFinder {
    priority_queue<int, vector<int>, greater<int>> f;
    priority_queue<int, vector<int>> b;

public:
    MedianFinder() : f(), b() {}

    void addNum(int num) {
        if (f.size() > b.size()) {
            f.push(num);
            int x = f.top();
            f.pop();
            b.push(x);
        } else {
            b.push(num);
            int x = b.top();
            b.pop();
            f.push(x);
        }
    }

    double findMedian() {
        if (f.size() == b.size()) {
            double a = f.top() + b.top();
            return a * 0.5;
        } else {
            return f.top();
        }
    }
};

int main() {
    auto mf = MedianFinder();
    mf.addNum(1);
    mf.addNum(2);
    assert(abs(mf.findMedian() - 1.5) <= 1e-5);
    mf.addNum(3);
    assert(abs(mf.findMedian() - 2.0) <= 1e-5);
    return 0;
}
