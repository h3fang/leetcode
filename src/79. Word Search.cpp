#include <cstdio>
#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool bt(vector<vector<char>>& board, int i, int j, int k, const string& word) {
        if (k == word.size()) {
            return true;
        }
        if (i < 0 || j < 0 || i >= board.size() || j >= board[0].size() || board[i][j] == 0) {
            return false;
        }

        if (board[i][j] != word[k]) {
            return false;
        }

        board[i][j] = 0;

        if (bt(board, i-1, j, k+1, word)) return true;
        if (bt(board, i+1, j, k+1, word)) return true;
        if (bt(board, i, j-1, k+1, word)) return true;
        if (bt(board, i, j+1, k+1, word)) return true;

        board[i][j] = word[k];

        return false;
    }

    bool exist(vector<vector<char>>& board, string word) {
        for (int i=0; i<board.size(); i++) {
            for (int j=0; j<board[0].size(); j++) {
                if (bt(board, i, j, 0, word)) return true;
            }
        }
        return false;
    }
};


int main() {
    vector<vector<char>> board = {
        {'A','B','C','E'},
        {'S','F','C','S'},
        {'A','D','E','E'}
    };
    string word = "ESEE";
    printf("%s\n", Solution().exist(board, word) ? "true" : "false");
    return 0;
}
