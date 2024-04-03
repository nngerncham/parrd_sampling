#include <vector>

template <typename DataType> class Sampler {
public:
  std::vector<DataType> static sample(std::vector<DataType> data, size_t k) {
    return std::vector<DataType>();
  }
};
