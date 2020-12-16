#include <vector>
#include <stack>
#include <cstdio>

using namespace std;

struct Info {
    int price;
    int ret;
};

class StockSpanner {
public:
    StockSpanner() {
    }

    int next(int price) {
        Info info{price, 1};
        while (!infos.empty()) {
            const auto i = infos.top();
            if (i.price > price) {
                break;
            }
            info.ret += i.ret;
            infos.pop();
        }
        infos.push(info);
        return info.ret;
    }
private:
    stack<Info> infos;
};

int main() {
    vector<int> prices = {28,14,28,35,46,53,66,80,87,88};

    auto s = new StockSpanner();
    for (auto p :prices) {
        printf("%d ", s->next(p));
    }
    printf("\n");
    return 0;
}