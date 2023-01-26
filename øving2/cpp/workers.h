#include <atomic>
#include <condition_variable>
#include <functional>
#include <mutex>
#include <queue>
#include <thread>
#include <vector>

class Workers {
public:
  Workers(int);
  ~Workers();

  int const get_count() const;
  void post(const std::function<void()> &);
  void post_timeout(const std::function<void()> &, int);
  void start();
  void stop();

private:
  const bool is_idle() const;

  bool started;
  bool should_stop;
  std::condition_variable stopper;

  int count;
  std::vector<std::thread> workers;

  std::queue<std::function<void()>> tasks;
  std::vector<std::thread> timeout_handles;
  std::atomic<int> timeout_handle_counter;
  std::mutex task_mutex;
  std::condition_variable task_cv;
};