#include <iostream>
#include <mutex>
#include <thread>
#include <vector>
#include <bits/stdc++.h>

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
  int from = 0;
  int to = 100;
  int threadcount = 5;

  std::vector<std::vector<int>> pools(threadcount);
  int thread = 0;

  for (int number = from; number <= to; number++) {
    pools[thread].push_back(number);
    thread++;

    if (thread == threadcount)
      thread = 0;
  }

  std::vector<std::thread> threads;
  std::mutex prime_mutex;
  std::vector<int> primes;

  for (int thread = 0; thread < threadcount; ++thread) {
    threads.emplace_back([thread, &pools, &primes, &prime_mutex] {
      for (auto number : pools[thread]) {
        if (is_prime(number)) {
          prime_mutex.lock();
          primes.push_back(number);
          prime_mutex.unlock();
        }
      }
    });
  }

  for (auto &thread : threads) {
    thread.join();
  }

  sort(primes.begin(), primes.end());

  std::cout << "[";
  for (auto prime : primes) {
    std::cout << " " << prime;
  }
  std::cout << " ]" << std::endl;
}
