#pragma once

#include <vector>

template <typename T> class Sampler {
public:
  std::vector<T> static sample(const std::vector<T> data, size_t k) {
    return std::vector<T>();
  }
};
