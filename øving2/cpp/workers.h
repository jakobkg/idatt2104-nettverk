#include <condition_variable>
#include <functional>
#include <queue>
#include <mutex>
#include <thread>
#include <vector>

class Workers {
public:
  Workers(int);
  ~Workers();

  int const get_count() const;
  void post(const std::function<void()> &);
  void start();
  void stop();

private:
  bool started;
  bool should_stop;

  int count;
  std::vector<std::thread> workers;

  std::queue<std::function<void()>> tasks;
  std::mutex task_mutex;
  std::condition_variable task_cv;
};