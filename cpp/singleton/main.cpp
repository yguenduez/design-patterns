#include <iostream>

struct ObjectToHold {
  void tellSomething() const { std::cout << "Tell something!" << std::endl; }
};

class Singleton {
private:
  ObjectToHold object;

public:
  Singleton(Singleton const &) = delete;
  Singleton &operator=(Singleton const &) = delete;

  static Singleton &getInstance() {
    static Singleton singleton;
    return singleton;
  };

  void printSomething() const { object.tellSomething(); }

private:
  Singleton() = default;
};

int main() {
  Singleton::getInstance().printSomething();

  // Compiler error, as the default CTOR is private
  // Singleton hi;
}
