#include "workers.h"
#include <iostream>
#include <thread>
#include <atomic>

int main(int argc, char *argv[]) {
  Workers workers(4);

  workers.start();

  std::atomic<int> sum(0);

  for (int i = 0; i < 5; ++i) {
    workers.post([&sum] { sum.fetch_add(1); });
  }

  workers.stop();
  
  std::cout << "Summerte " << sum << std::endl;

  return 0;
}
