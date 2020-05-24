#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>
#include <cstdio>

using namespace std;

class FooBar {
private:
    int n;
    mutex m;
    condition_variable cv;
    bool first = true;

public:
    FooBar(int n) {
        this->n = n;
    }

    void foo(function<void()> printFoo) {
        for (int i = 0; i < n; i++) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this](){return first==true;});
            lk.unlock();
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
            first = false;
            cv.notify_one();
        }
    }

    void bar(function<void()> printBar) {
        for (int i = 0; i < n; i++) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this](){return first==false;});
            lk.unlock();
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
            first = true;
            cv.notify_one();
        }
    }
};

int main() {
    FooBar fb(10);
    thread([&fb]() { fb.foo([]() { printf("foo\n"); }); }).detach();
    thread([&fb]() { fb.bar([]() { printf("bar\n"); }); }).detach();
    return 0;
}