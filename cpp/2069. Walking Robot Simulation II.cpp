#include <string>
#include <vector>

using namespace std;

class Robot {
    int width;
    int height;
    int total;
    int index;
    bool moved;

public:
    Robot(int width, int height)
        : width(width), height(height), total(2 * width + 2 * height - 4), index(0), moved(false) {}

    void move(int num) {
        index = (index + num) % total;
        moved = true;
    }

    vector<int> getPos() {
        if (index < width) {
            return {index, 0};
        } else if (index < width + height - 1) {
            return {width - 1, index - (width -1)};
        } else if (index < 2 * width + height - 2) {
            return {width - 1 - (index - (width + height - 2)), height - 1};
        } else {
            return {0, height - 1 - (index - (2 * width + height - 3))};
        }
    }

    string getDir() {
        if (!moved) {
            return "East";
        }
        if (index == 0) {
            return "South";
        }
        else if (index < width) {
            return "East";
        } else if (index < width + height - 1) {
            return "North";
        } else if (index < 2 * width + height - 2) {
            return "West";
        } else {
            return "South";
        }
    }
};

int main() {
    Robot robot(6, 3); // 初始化网格图，机器人在 (0, 0) ，朝东。
    robot.move(2);     // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
    robot.move(2);     // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
    robot.getPos();    // 返回 [4, 0]
    robot.getDir();    // 返回 "East"
    robot.move(2);     // 朝东移动 1 步到达 (5, 0) ，并朝东。
                       // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
                       // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
    robot.move(1);     // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
    robot.move(4);     // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
                       // 然后，移动 4 步到 (1, 2) ，并朝西。
    robot.getPos();    // 返回 [1, 2]
    robot.getDir();    // 返回 "West"
}
