#include <cstdio>
#include <string>

using namespace std;

class Solution {
public:
    bool isPalindrome(string s) {
        if(s.empty()) {return true;}

        char *begin = (char*)s.c_str(), *end = begin + (s.length()-1);

        while(begin <= end){
            while(!isAlphanumeric(begin) && (begin < end)) {begin++;}
            while(!isAlphanumeric(end) && (begin < end)) {end--;}

            if((*begin == *end)) {begin++; end--;}
            else return false;
        }

        return true;
    }

    inline bool isAlphanumeric(char *c){
        if((*c >= 'A') && (*c <= 'Z')) {*c += 32; return true;}
        else return ( (*c >= 'a') && (*c <= 'z') ) || ( (*c >= '0') && (*c <= '9') );
    }
};

int main() {
    printf("%d\n", Solution().isPalindrome("A man, a plan, a canal: Panama"));
    return 0;
}