#include <cstdio>
#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

class H2O {
public:
    H2O() {}

    void hydrogen(function<void()> releaseHydrogen) {
        unique_lock<mutex> lk(m);
        cv.wait(lk, [this]{return h<2;});
        // releaseHydrogen() outputs "H". Do not change or remove this line.
        releaseHydrogen();
        h++;
        cv.notify_all();
    }

    void oxygen(function<void()> releaseOxygen) {
        unique_lock<mutex> lk(m);
        cv.wait(lk, [this]{return h==2;});
        // releaseOxygen() outputs "O". Do not change or remove this line.
        releaseOxygen();
        h = 0;
        cv.notify_all();
    }

private:
    mutex m;
    condition_variable cv;
    int h{0};
};

int main() {
    string h2o = "HOOOOHHHHHHH";
    H2O builder;
    const auto releaseHydrogen = [&] { builder.hydrogen([] { fprintf(stdout, "H"); }); };
    const auto releaseOxygen = [&] { builder.oxygen([] { fprintf(stdout, "O"); }); };
    vector<thread> threads;
    for (char c : h2o) {
        if (c == 'H')
            threads.push_back(thread(releaseHydrogen));
        else if (c == 'O')
            threads.push_back(thread(releaseOxygen));
    }

    for (auto &t : threads) {
        t.join();
    }
    printf("\n");
    return 0;
}