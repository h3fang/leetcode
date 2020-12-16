#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>
#include <cstdio>

using namespace std;

class Foo {
  public:
    Foo() {}

    void first(function<void()> printFirst) {
        unique_lock<mutex> lk(m);
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        print++;
        lk.unlock();
        p.notify_all();
    }

    void second(function<void()> printSecond) {
        unique_lock<mutex> lk(m);
        p.wait(lk, [this]() { return print == 1; });
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        print++;
        lk.unlock();
        p.notify_all();
    }

    void third(function<void()> printThird) {
        unique_lock<mutex> lk(m);
        p.wait(lk, [this]() { return print == 2; });
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
    }

  private:
    mutex m;
    condition_variable p;
    int print = 0;
};

int main() {
    Foo foo;
    thread([&foo]() { foo.second([]() { printf("printsecond\n"); }); }).detach();
    thread([&foo]() { foo.first([]() { printf("printFirst\n"); }); }).detach();
    thread([&foo]() { foo.third([]() { printf("printThird\n"); }); }).detach();
    return 0;
}