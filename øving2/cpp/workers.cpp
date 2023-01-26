#include "workers.h"
#include <iostream>

Workers::Workers(int count)
    : tasks(), task_mutex(), workers(), count(count), task_cv(),
      started(false), should_stop(false) {}

const int Workers::get_count() const { return count; }

void Workers::post(const std::function<void()> &func) {
  if (started) {
    std::unique_lock<std::mutex> lock(task_mutex);
    tasks.push(func);
  }

  task_cv.notify_one();
}

void Workers::start() {
  for (int _ = 0; _ < count; ++_) {
    workers.emplace_back([this] {
      while (true) {
        std::function<void()> task;
        {
          std::unique_lock<std::mutex> lock(task_mutex);
          task_cv.wait(lock, [this] {
            return this->should_stop || !this->tasks.empty();
          });

          if (this->should_stop)
            return;

          task = this->tasks.front();
          this->tasks.pop();
        }

        if (task) {
          task();
        }
      }
    });
  }

  started = true;
}

void Workers::stop() {
  should_stop = true;
  task_cv.notify_all();

  for (auto &worker : workers) {
    worker.join();
  }
}

Workers::~Workers() {}