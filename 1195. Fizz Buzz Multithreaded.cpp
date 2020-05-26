#include <condition_variable>
#include <cstdio>
#include <functional>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

class FizzBuzz {
  private:
    int n, i = 1;
    mutex m;
    condition_variable cv;

  public:
    FizzBuzz(int n) { this->n = n; }

    // printFizz() outputs "fizz".
    void fizz(function<void()> printFizz) {
        while (1) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return i % 3 == 0 && i % 5 != 0 || i > n; });
            if (i > n) {
                return;
            }
            i++;
            printFizz();
            cv.notify_all();
        }
    }

    // printBuzz() outputs "buzz".
    void buzz(function<void()> printBuzz) {
        while (1) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return i % 3 != 0 && i % 5 == 0 || i > n; });
            if (i > n) {
                return;
            }
            i++;
            printBuzz();
            cv.notify_all();
        }
    }

    // printFizzBuzz() outputs "fizzbuzz".
    void fizzbuzz(function<void()> printFizzBuzz) {
        while (1) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return i % 3 == 0 && i % 5 == 0 || i > n; });
            if (i > n) {
                return;
            }
            i++;
            printFizzBuzz();
            cv.notify_all();
        }
    }

    // printNumber(x) outputs "x", where x is an integer.
    void number(function<void(int)> printNumber) {
        while (1) {
            unique_lock<mutex> lk(m);
            cv.wait(lk, [this] { return i % 3 != 0 && i % 5 != 0 || i > n; });
            if (i > n) {
                return;
            }
            printNumber(i++);
            cv.notify_all();
        }
    }
};

int main() {
    const int N = 15;
    FizzBuzz fb(N);
    const auto s = [] { this_thread::sleep_for(chrono::milliseconds(rand() % 10)); };
    const auto fizz = [&] {
        for (int i = 0; i < N; i++) {
            s();
            fb.fizz([] { fprintf(stdout, "fizz "); });
        }
    };
    const auto buzz = [&] {
        for (int i = 0; i < N; i++) {
            s();
            fb.buzz([] { fprintf(stdout, "buzz "); });
        }
    };
    const auto fizzbuzz = [&] {
        for (int i = 0; i < N; i++) {
            s();
            fb.fizzbuzz([] { fprintf(stdout, "fizzbuzz "); });
        }
    };
    const auto number = [&] {
        for (int i = 0; i < N; i++) {
            s();
            fb.number([](int n) { fprintf(stdout, "%d ", n); });
        }
    };
    thread threads[] = {thread(fizzbuzz), thread(fizz), thread(buzz), thread(number)};
    for (auto &t : threads) {
        t.join();
    }
    printf("\n");
    return 0;
}