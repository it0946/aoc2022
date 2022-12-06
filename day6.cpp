#include <iostream>
#include <set>
#include <string>
#include <fstream>

using namespace std;

int main()
{
    fstream in{"inputs/day6"};
    string input;
    set<char> res;
    int count = 0;
    int i = 0;

    getline(in, input);

    /* p2 */
    // for (; i < input.size() && count < 14; i++) {
    //     if (!res.insert(input[i]).second) {
    //         i = i - count;
    //         count = 0;
    //         res.clear();
    //         continue;
    //     }
    //     count += 1;
    // }

    /* p1 */
    for (; i < input.size() && count < 4; ++i) {
        if (!res.insert(input[i]).second) {
            count = 0;
            res.clear();
            continue;
        }
        count += 1;
    }

    cout << i << "\n";
}