#include <fstream>
#include <vector>
#include <deque>
#include <vector>
#include <iostream>

using namespace std;

int main()
{
    fstream in{"inputs/day5"};
    vector<deque<char>> p1{9};
    string buf;

    while (true) {
        getline(in, buf);
        if (buf.starts_with(" 1")) break;
        for (int i = 0; i < 9; i++) {
            char c = buf[1 + i * 4];
            if (c != ' ')
                p1[i].push_front(c);
        }
    }

    getline(in, buf);

    auto p2 = p1;
    vector<char> tmp;
    while (getline(in, buf)) {
        int n, from, to;
        sscanf(buf.c_str(), "move %d from %d to %d", &n, &from, &to);

        for (; n > 0; n -= 1) {
            p1[to - 1].push_back(p1[from - 1].back());
            tmp.push_back(p2[from - 1].back());
            p1[from - 1].pop_back();
            p2[from - 1].pop_back();
        }

        p2[to - 1].insert(p2[to - 1].end(), tmp.rbegin(), tmp.rend());
        tmp.clear();
    }

    for (const auto& v : p2)
        cout << v.back();
    cout << "\n";
    for (const auto& v : p1)
        cout << v.back();
    cout << "\n";

    in.close();
}
