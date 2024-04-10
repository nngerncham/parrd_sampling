#include "../parlay/delayed_sequence.h"
#include "../parlay/parallel.h"
#include "../parlay/primitives.h"
#include "../parlay/sequence.h"
#include "../parlay/utilities.h"

#include "sampler_model.h"

#include <algorithm>
#include <atomic>
#include <cstddef>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <map>
#include <random>
#include <stdatomic.h>
#include <utility>
#include <vector>

#define PREFIX_DIVISOR 20

template <typename T> class SeqPermutationSampler : Sampler<T> {
public:
  std::vector<T> static sample(const std::vector<T> &data, size_t k) {
    std::random_device rd;
    std::mt19937 gen(rd());

    std::vector<T> ans;
    ans.reserve(k);
    std::map<size_t, size_t> actual_target;
    for (size_t i = 0; i < data.size(); i++) {
      std::uniform_int_distribution<size_t> dis(i, data.size() - 1);
      size_t rd_idx = dis(gen);

      if (rd_idx == i) {
        // picked itself, copy
        ans.push_back(data[rd_idx]);
      } else if (actual_target.find(rd_idx) == actual_target.end()) {
        // swap target never picked, point to where element was swapped to
        ans.push_back(data[rd_idx]);
        actual_target.insert(std::pair<size_t, size_t>(rd_idx, i));
      } else {
        // swap target picked before, go get it from swapped location
        ans.push_back(data[actual_target[rd_idx]]);
        actual_target[rd_idx] = i;
      }
    }

    return ans;
  }
};

template <typename T> class SeqPermutationFullSampler : Sampler<T> {
public:
  std::vector<T> static sample(const std::vector<T> &data, size_t k) {
    // prepping the RNG
    std::random_device rd;
    std::mt19937 gen(rd());

    // shuffling
    size_t n = data.size();
    std::vector<T> ans(data); // do full copy because swap anyway
    for (size_t i = 0; i < k; i++) {
      std::uniform_int_distribution<size_t> dis(i, n - 1);
      size_t swap_idx = dis(gen);
      std::swap(ans[i], ans[swap_idx]);
    }

    // truncate
    ans.resize(k);
    return ans;
  }
};

template <typename T> class ParPermutationSampler : Sampler<T> {
public:
  std::vector<T> static sample(const std::vector<T> &data, size_t k) {
    size_t n = data.size();

    // generates swap targets
    parlay::random_generator gen(time(NULL));
    parlay::sequence<size_t> swap_target =
        parlay::tabulate(data.size(), [&](size_t i) {
          std::uniform_int_distribution<size_t> dis(i, n - 1);
          auto r = gen[i];
          return dis(r);
        });

    // reserve and commit
    std::vector<T> ans(data);
    permute(ans, k, swap_target);
    ans.resize(k);
    return ans;
  }

  void static permute(std::vector<T> &ans, size_t k,
                      parlay::sequence<size_t> &swap_target) {
    size_t n = ans.size();
    auto reservations =
        parlay::tabulate(n, [&](size_t i) { return std::atomic_size_t(n); });
    auto reserve = [&](size_t i) {
      fetch_min(reservations[i], i);
      fetch_min(reservations[swap_target[i]], i);
    };
    auto commit = [&](size_t i) {
      size_t swap_idx = swap_target[i];
      if (reservations[i].load() == i && reservations[swap_idx].load() == i) {
        std::swap(ans[i], ans[swap_idx]);
        reservations[swap_idx].store(n);
        return false;
      } else {
        return true;
      }
    };

    parlay::sequence<size_t> idx_left =
        parlay::tabulate(k, [&](size_t i) { return i; });
    size_t prefix_size =
        std::max<size_t>(idx_left.size() / PREFIX_DIVISOR, PREFIX_DIVISOR);

    while (!idx_left.empty()) {
      // reserve phase
      size_t to_reserve_size = std::min<size_t>(idx_left.size(), prefix_size);
      parlay::parallel_for(0, to_reserve_size,
                           [&](size_t i) { reserve(idx_left[i]); });

      // commit + repacking phase
      parlay::sequence<size_t> new_idx_left =
          parlay::filter(idx_left.head(to_reserve_size),
                         [&](size_t idx) { return commit(idx); });
      if (idx_left.size() > to_reserve_size) { // prefix shorter # reserved
        new_idx_left.append(idx_left.tail(idx_left.size() - to_reserve_size));
      }

      idx_left = new_idx_left;
      prefix_size =
          std::max<size_t>(idx_left.size() / PREFIX_DIVISOR, PREFIX_DIVISOR);
    }
  }

private:
  void static fetch_min(std::atomic_size_t &var, size_t new_value) {
    size_t current_value = var.load();
    while (new_value < current_value &&
           !var.compare_exchange_weak(current_value, new_value)) {
    }
  }
};

template <typename T>
class ParPermutationFullSampler : public ParPermutationSampler<T> {
public:
  std::vector<T> static sample(const std::vector<T> &data, size_t k) {
    size_t n = data.size();

    // generates swap targets
    parlay::random_generator gen(time(NULL));
    parlay::sequence<size_t> swap_target =
        parlay::tabulate(data.size(), [&](size_t i) {
          std::uniform_int_distribution<size_t> dis(i, n - 1);
          auto r = gen[i];
          return dis(r);
        });

    std::vector<T> ans(data);
    ParPermutationSampler<T>::permute(ans, n, swap_target);
    ans.resize(k);
    return ans;
  }
};
