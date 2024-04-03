#include "parlay/primitives.h"
#include "parlay/sequence.h"
#include <iostream>

int main(int argc, char *argv[]) {
  auto arr = parlay::tabulate(100, [](int i) { return i; });
  for (int i = 0; i < 100; i++) {
    std::cout << arr[i] << " ";
  }
  std::cout << "\n";
  return 0;
}
