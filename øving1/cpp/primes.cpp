#include <iostream>
#include <thread>
#include <vector>

bool is_prime(int x) {
    // Ingen partall er primtall (unntatt 2)
    if ((x % 2 == 0 && x != 2) || x < 2) {
        return false;
    }

    for (int factor = 3; factor <= x / 2; factor += 2) {
        if (x % factor == 0) {
            return false;
        }
    }

    return true;
}

int main(int argc, char *argv[]) {
    std::vector<std::thread> threads;

    for (size_t i = 1; i <= 25; i++)
        std::cout << i << ": " << is_prime(i) << std::endl;
}
