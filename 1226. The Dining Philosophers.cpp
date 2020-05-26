#include <cstdio>
#include <condition_variable>
#include <functional>
#include <mutex>
#include <thread>
#include <vector>
#include <map>

using namespace std;

const int N = 5;

class Fork
{
public:
    mutex m;
    condition_variable cv;
    bool owned = false;
public:
    void pick() {
        unique_lock<mutex> lk(m);
        cv.wait(lk, [this]{return !owned;});
        owned = true;
        cv.notify_all();
    }

    void put() {
        owned = false;
        cv.notify_all();
    }
};

class Philosopher
{
public:
    void assignForks(Fork* left, Fork* right) {
        leftFork = left;
        rightFork = right;
    }

public:
    Fork *leftFork, *rightFork;
};

class DiningPhilosophers {
private:
    mutex m;
    condition_variable cv;
    vector<Fork> forks{N};
    vector<Philosopher> philosophers{N};

public:
    DiningPhilosophers() {
        for (int i = 0; i < N; i++) {
            philosophers[i].assignForks(&forks[i-1<0 ? N-1 : i-1], &forks[i]);
        }
    }

    void wantsToEat(int philosopher,
                    function<void()> pickLeftFork,
                    function<void()> pickRightFork,
                    function<void()> eat,
                    function<void()> putLeftFork,
                    function<void()> putRightFork) {
        Philosopher& p = philosophers[philosopher];
        unique_lock<mutex> lk(m);
        cv.wait(lk, [&p] { return !p.leftFork->owned && !p.rightFork->owned; });
        p.leftFork->pick();
        pickLeftFork();
        p.rightFork->pick();
        pickRightFork();
        eat();
        p.leftFork->put();
        putLeftFork();
        p.rightFork->put();
        putRightFork();
        cv.notify_all();
    }
};

int main() {
    int n = 3;
    DiningPhilosophers dp;
    map<thread::id, int> thread_ids;
    const auto s = [] { this_thread::sleep_for(chrono::milliseconds(rand() % 10));};
    const auto pl = [&] { s(); fprintf(stdout, "[%d,1,1]", thread_ids[this_thread::get_id()]); };
    const auto pr = [&] { s(); fprintf(stdout, "[%d,2,1]", thread_ids[this_thread::get_id()]); };
    const auto e = [&] { s(); fprintf(stdout, "[%d,0,3]", thread_ids[this_thread::get_id()]); };
    const auto pul = [&] { s(); fprintf(stdout, "[%d,1,2]", thread_ids[this_thread::get_id()]); };
    const auto pur = [&] { s(); fprintf(stdout, "[%d,2,2]", thread_ids[this_thread::get_id()]); };
    thread threads[N];
    for (int i = 0; i < N; i++) {
        threads[i] = thread([&, i] {
            for (int j = 0; j < n; j++) {
                s();
                dp.wantsToEat(i, pl, pr, e, pul, pur);
            }
        });
        thread_ids[threads[i].get_id()] = i;
    };
    for (auto &t : threads) {
        t.join();
    }
    printf("\n");
    return 0;
}