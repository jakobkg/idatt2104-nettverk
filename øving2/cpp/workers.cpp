#include "workers.h"
#include <iostream>

Workers::Workers(int count)
    : tasks(), task_mutex(), task_cv(), timeout_handles(),
      timeout_handle_counter(0), workers(), count(count), started(false),
      should_stop(false), stopper() {}

const int Workers::get_count() const { return count; }

void Workers::post(const std::function<void()> &func) {
  if (started) {
    std::unique_lock<std::mutex> lock(task_mutex);
    tasks.push(func);
  }

  task_cv.notify_all();
}

void Workers::post_timeout(const std::function<void()> &func, int sleep_ms) {
  timeout_handle_counter++;
  timeout_handles.emplace_back([this, sleep_ms, func] {
    std::this_thread::sleep_for(std::chrono::milliseconds(sleep_ms));
    this->post(func);
    this->timeout_handle_counter--;
  });
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

          if (!this->tasks.empty()) {
            task = this->tasks.front();
            this->tasks.pop();
          }
        }

        if (task) {
          task();
          this->stopper.notify_one();
        }
      }
    });
  }

  started = true;
}

void Workers::stop() {
  {
    std::unique_lock<std::mutex> lock(task_mutex);

    stopper.wait(lock, [this] { return this->is_idle(); });
  }

  should_stop = true;
  task_cv.notify_all();

  for (auto &worker : workers) {
    worker.join();
  }

  for (auto &handle : timeout_handles) {
    handle.join();
  }
}

const bool Workers::is_idle() const {
  return tasks.empty() && timeout_handle_counter == 0;
}

Workers::~Workers() {}