#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

#define FLAG_S 1
#define FLAG_P 2

#define M 1000

int dx[4] = {1, 0, -1, 0};
int dy[4] = {0, 1, 0, -1};

void show_help(char *name) {
    cout << "Usage: " << name << " [options] <number>" << endl;
    cout << "\nOptions:" << endl;
    cout << "  -s  : generate board by shuffle (solvable)" << endl;
    cout << "  -p  : generate board as a permutaion (might be unsolvable)" << endl;
}

void show_grid(vector<vector<int>> &grid) {
    cout << grid.size() << endl;
    for (int i = 0; i < grid.size(); i++) {
        for (int j = 0; j < grid.size(); j++) {
            cout << grid[i][j] << " ";
        }
        cout << endl;
    }
}

vector<vector<int>> start_grid(int n) {
    vector<vector<int>> grid(n, vector<int>(n, 0));

    int dir = 0; 
    int cur = 1;
    int i = 0;
    int j = 0;

    while (cur < n * n) {
        grid[i][j] = cur++;

        i += dy[dir % 4];
        j += dx[dir % 4];

        if (i < 0 || i == grid.size() || j < 0 || j == grid.size() || grid[i][j]) {
            i -= dy[dir % 4];
            j -= dx[dir % 4];
            dir++;
            i += dy[dir % 4];
            j += dx[dir % 4];
        }
    }
    return grid;
}

void shuffle(int n) {
    vector<vector<int>> grid = start_grid(n);

    for (int _ = 0; _ < M * n; _++) {
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 0) {
                    int r = rand() % 4;
                    int x = j + dx[r];
                    int y = i + dy[r];
                    if (x >= 0 && x < n && y >= 0 && y < n) {
                        swap(grid[i][j], grid[y][x]);
                    }
                    goto main_loop;
                }
            }
        }
        main_loop:;
    }
    show_grid(grid);
}

void permutation(int n) {
    vector<vector<int>> grid(n, vector<int>(n));

    vector<int> v(n * n);
    for (int i = 0; i < n * n; i++) {
        v[i] = i;
    }

    for (int i = 0; i < n * n; i++) {
        int r = rand() % (n * n - i);
        grid[i / n][i % n] = v[r];
        swap(v[r], v.back());
        v.pop_back();
    } 

    show_grid(grid);
}

int main(int ac, char **av) {
    if (ac == 1) {
        show_help(av[0]);
        return 1;
    }

    srand(time(NULL));

    int flag = 0;
    int num = -1;
    for (int i = 1; i < ac; i++) {
        if (av[i][0] == '-') {
            if (av[i][1] == 's') {
                flag |= FLAG_S;
            } else if (av[i][1] == 'p') {
                flag |= FLAG_P;
            } else {
                show_help(av[0]);
                return 1;
            }
        }
        else {
            num = atoi(av[i]);
        }
    }
    if (num <= 0) {
        cerr << "Invalid number: " << num << endl;
        return 1;
    }
    if (flag == (FLAG_P | FLAG_S)) {
        cerr << "A un moment il faut choisir" << endl;
    }

    if (flag & FLAG_P) {
        cerr << "Generating permutation" << endl;
        permutation(num);
    } else {
        cerr << "Generating solvable map" << endl;
        shuffle(num);
    }
}