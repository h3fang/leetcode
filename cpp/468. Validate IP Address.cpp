#include <cstdio>
#include <regex>
#include <string>

using namespace std;

class Solution {
public:
    string validIPAddress(string IP) {
        for (char &ch : IP) {
            ch = std::tolower(ch);
        }
        if (regex_match(IP, regex("^((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])(\\.(?!$)|$)){4}$"))) {
            return "IPv4";
        } else if (regex_match(IP, regex("^(([a-f0-9]{1,4})(:(?!$)|$)){8}$"))) {
            return "IPv6";
        }
        return "Neither";
    }
};

int main() {
    string ip = "255.055.255.255";
    printf("%s\n", Solution().validIPAddress(ip).data());
    return 0;
}