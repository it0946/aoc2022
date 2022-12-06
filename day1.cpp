#include <string>
#include <fstream>
#include <algorithm>
#include <vector>
#include <fmt/core.h>

std::string get_input()
{
    std::ifstream file{"inputs/day1"};
    std::istreambuf_iterator<char> begin{file}, end;
    std::string res{begin, end};
    file.close();
    return res;
}

void solve(const std::string& input)
{
    std::vector<int> res{0};
    size_t i = 0, pos = 0, prev = 0;

    while ((pos = input.find('\n', prev)) != input.npos) {
        std::string curr = input.substr(prev, pos - prev);

        if (!curr.empty()) {
            res[i] += std::stol(curr);
        } else {
            i += 1;
            res.push_back(0);
        }

        prev = pos + 1;
    }

    std::sort(res.begin(), res.end());

    size_t l = res.size();
    fmt::print("part1: {}\npart2: {}\n", res.back(), res[l - 1] + res[l - 2] + res[l - 3]);
}

int main()
{
    solve(get_input());
}