#include <cassert>
#include <cstdio>
#include <vector>

using namespace std;

class Iterator {
    const vector<int> &nums;
    int i = 0;

public:
    Iterator(const vector<int> &nums) : nums(nums) {}
    Iterator(const Iterator &iter) : nums(iter.nums), i(iter.i) {}
    // Returns the next element in the iteration.
    int next() {
        return nums[i++];
    }
    // Returns true if the iteration has more elements.
    bool hasNext() const {
        return i < nums.size();
    }
};

class PeekingIterator : public Iterator {
    int current = 0;

public:
    PeekingIterator(const vector<int> &nums) : Iterator(nums) {
        current = Iterator::next();
    }

    // Returns the next element in the iteration without advancing the iterator.
    int peek() {
        return current;
    }

    // hasNext() and next() should behave the same as in the Iterator interface.
    // Override them if needed.
    int next() {
        auto r = current;
        if (Iterator::hasNext()) {
            current = Iterator::next();
        } else {
            current = 0;
        }
        return r;
    }

    bool hasNext() const {
        return current != 0;
    }
};

int main() {
    vector<int> nums = {1, 2, 3};
    auto pi = PeekingIterator(nums);
    assert(pi.next() == 1);
    assert(pi.peek() == 2);
    assert(pi.next() == 2);
    assert(pi.next() == 3);
    assert(pi.hasNext() == false);
    return 0;
}