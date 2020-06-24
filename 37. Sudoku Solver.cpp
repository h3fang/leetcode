#include <cstdio>
#include <algorithm>
#include <vector>
#include <chrono>

using namespace std;

void printSudoku(const vector<vector<char>>& board) {
    int r = 0, c = 0;
    for (const auto& row : board) {
        if (r%3 == 0) {
            printf("+-------+-------+-------+\n");
        }
        c = 0;
        for (char e : row) {
            if (c%3 == 0) {
                printf("| ");
            }
            printf("%c ", e);
            c++;
        }
        printf("|\n");
        r++;
    }
    printf("+-------+-------+-------+\n");
}

class Solution {
private:
    bool candidates[9][9][9];

public:
    bool isValid(const vector<vector<char>>& board, int i, int j, char val) {
        if (board[i][j] != '.') { return board[i][j] == val; }
        for (int k = 0; k<9; k++) {
            if (board[i][k] == val) return false;
            if (board[k][j] == val) return false;
        }
        for (int k = i-(i%3); k<i-(i%3)+3; k++) {
            for (int m = j-(j%3); m<j-(j%3)+3; m++) {
                if (board[k][m] == val) return false;
            }
        }
        return true;
    }

    void updateCandidate(const vector<vector<char>>& board, int i, int j) {
        if (board[i][j] == '.') {
            for (auto& c : candidates[i][j]) {
                c = true;
            }
            for (int k = 0; k<9; k++) {
                if (board[i][k] != '.') candidates[i][j][board[i][k] - '1'] = false;
                if (board[k][j] != '.') candidates[i][j][board[k][j] - '1'] = false;
            }
            for (int k = i-(i%3); k<i-(i%3)+3; k++) {
                for (int m = j-(j%3); m<j-(j%3)+3; m++) {
                    if (board[k][m] != '.') candidates[i][j][board[k][m] - '1'] = false;
                }
            }
        } else {
            for (auto& c : candidates[i][j]) {
                c = false;
            }
        }
    }

    void updateCandidate(const vector<vector<char>>& board) {
        for (int i = 0; i<9; i++) {
            for (int j = 0; j<9; j++) {
                updateCandidate(board, i, j);
            }
        }
    }

    int setSoleCandidate(vector<vector<char>>& board) {
        int updated = 0;
        for (int i=0; i<9; i++) {
            for (int j=0; j<9; j++) {
                if (board[i][j] != '.') { continue; }
                int size = 0, c = 0;
                for (int k=0; k<9; k++) {
                    if (candidates[i][j][k]) {
                        size++;
                        c = k;
                        if (size > 1) {
                            break;
                        }
                    }
                }
                if (size == 1) {
                    board[i][j] = c + '1';
                    updated++;
                }
            }
        }
        return updated;
    }

    int setUniqueCandidate(vector<vector<char>>& board) {
        for (int i=0; i<9; i++) {
            for (int j=0; j<9; j++) {
                if (board[i][j] != '.') { continue; }
                for (int k=0; k<9; k++) {
                    if (candidates[i][j][k]) {
                        bool unique = true;
                        // row
                        for (int m=0; m<9; m++) {
                            if (m != j && board[i][m] == '.' && candidates[i][m][k]) {
                                unique = false;
                                break;
                            }
                        }
                        if (unique) {
                            board[i][j] = k + '1';
                            return 1;
                        }

                        // column
                        unique = true;
                        for (int m=0; m<9; m++) {
                            if (m != i && board[m][j] == '.' && candidates[m][j][k]) {
                                unique = false;
                                break;
                            }
                        }
                        if (unique) {
                            board[i][j] = k + '1';
                            return 1;
                        }

                        // block
                        unique = true;
                        for (int m = i-(i%3); m<i-(i%3)+3; m++) {
                            for (int n = j-(j%3); n<j-(j%3)+3; n++) {
                                if (board[m][n] == '.' && candidates[m][n][k]) {
                                    unique = false;
                                    break;
                                }
                            }
                            if (!unique) {
                                break;
                            }
                        }
                        if (unique) {
                            board[i][j] = k + '1';
                            return 1;
                        }
                    }
                }
            }
        }
        return 0;
    }

    bool bt(vector<vector<char>>& board, vector<pair<pair<int, int>, vector<char>>> & remaining, int n) {
        if (n == remaining.size()) { return true; }
        auto [p, s] = remaining[n];
        for (char val : s) {
            if (isValid(board, p.first, p.second, val)) {
                board[p.first][p.second] = val;
                if (bt(board, remaining, n+1)) {
                    return true;
                } else {
                    board[p.first][p.second] = '.';
                }
            }
        }
        return false;
    }

    void solveSudoku(vector<vector<char>>& board) {
        while (true) {
            updateCandidate(board);
            int updated = setSoleCandidate(board);
            if (updated) { updateCandidate(board); }
            updated += setUniqueCandidate(board);
            if (updated == 0) {
                break;
            }
        };

        vector<pair<pair<int, int>, vector<char>>> remaining;

        for (int i=0; i<9; i++) {
            for (int j=0; j<9; j++) {
                if (board[i][j] != '.') { continue; }
                pair<pair<int, int>, vector<char>> p{{i,j}, {}};
                for (int k=0; k<9; k++) {
                    if (candidates[i][j][k]) {
                        p.second.push_back(k + '1');
                    }
                }
                remaining.push_back(p);
            }
        }

        sort(remaining.begin(), remaining.end(), [](const auto& l, const auto& r){
            return l.second.size() < r.second.size();
        });

        bt(board, remaining, 0);
    }
};

int main() {
    vector<vector<char>> board = {
        {'.','.','9','7','4','8','.','.','.'},
        {'7','.','.','.','.','.','.','.','.'},
        {'.','2','.','1','.','9','.','.','.'},
        {'.','.','7','.','.','.','2','4','.'},
        {'.','6','4','.','1','.','5','9','.'},
        {'.','9','8','.','.','.','3','.','.'},
        {'.','.','.','8','.','3','.','2','.'},
        {'.','.','.','.','.','.','.','.','6'},
        {'.','.','.','2','7','5','9','.','.'}
    };
    auto start = chrono::high_resolution_clock::now();
    Solution().solveSudoku(board);
    auto t = chrono::duration_cast<chrono::microseconds>(chrono::high_resolution_clock::now() - start);
    printSudoku(board);
    printf("time: %ld microseconds\n", t.count());
    return 0;
}