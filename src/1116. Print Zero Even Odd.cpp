#include <condition_variable>
#include <cstdio>
#include <functional>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

class ZeroEvenOdd {
  private:
    int n, c = 1;
    bool is_zero = true;
    mutex m;
    condition_variable cv;

  public:
    ZeroEvenOdd(int n) { this->n = n; }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        while (c <= n) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return is_zero; });
            if (c <= n) {
                printNumber(0);
            }
            is_zero = false;
            lk.unlock();
            cv.notify_all();
        }
    }

    void even(function<void(int)> printNumber) {
        while (c <= n) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return !is_zero && c % 2 == 0; });
            if (c <= n) {
                printNumber(c);
                c++;
            }
            is_zero = true;
            lk.unlock();
            cv.notify_all();
        }
    }

    void odd(function<void(int)> printNumber) {
        while (c <= n) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return !is_zero && c % 2 == 1; });
            if (c <= n) {
                printNumber(c);
                c++;
            }
            is_zero = true;
            lk.unlock();
            cv.notify_all();
        }
    }
};

int main() {
    const int N = 10;
    ZeroEvenOdd zeo(N);
    const auto printnum = [](int n) { printf("%d\n", n); };
    vector<thread> threads;
    threads.push_back(thread([&] {
        this_thread::sleep_for(chrono::milliseconds(rand() % 100));
        zeo.zero(printnum);
    }));
    threads.push_back(thread([&] {
        this_thread::sleep_for(chrono::milliseconds(rand() % 100));
        zeo.even(printnum);
    }));
    threads.push_back(thread([&] {
        this_thread::sleep_for(chrono::milliseconds(rand() % 100));
        zeo.odd(printnum);
    }));
    for (auto &t : threads) {
        t.join();
    }
    return 0;
}