#include "workers.h"
#include <atomic>
#include <iostream>
#include <thread>

int main(int argc, char *argv[]) {
  Workers workers(4);

  workers.start();

  workers.post_timeout([] { std::cout << "A" << std::endl; }, 2000);
  workers.post_timeout([] { std::cout << "B" << std::endl; }, 1000);

  workers.stop();

  return 0;
}
