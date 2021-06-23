#include <iostream>
#include <memory>
#include <stdexcept>
#include <vector>

class Prototype {
public:
  virtual std::unique_ptr<Prototype> clone() const = 0;
  virtual void doOtherThings() const = 0;
  virtual void printInternalState() const = 0;

  virtual ~Prototype() = default;
};

struct ConcretePrototypeOne : Prototype {
  int internalState{};

  std::unique_ptr<Prototype> clone() const override {
    auto clone = std::make_unique<ConcretePrototypeOne>();
    *clone = *this;
    return clone;
  }

  void doOtherThings() const override {
    std::cout << "Ok I am doing something else than cloning myself"
              << std::endl;
  }

  void printInternalState() const override {
    std::cout << internalState << std::endl;
  }
};

struct ConcretePrototypeTwo : Prototype {
  int internalState{};

  std::unique_ptr<Prototype> clone() const override {
    auto clone = std::make_unique<ConcretePrototypeTwo>();
    *clone = *this;
    return clone;
  }

  void doOtherThings() const override {
    std::cout
        << "Just copying myself is too easy. I also pretend to be my daddy"
        << std::endl;
  }
  void printInternalState() const override {
    std::cout << internalState << std::endl;
  }
};

enum class PrototypeType { One, Two };

struct PrototypeFactory {
  static std::unique_ptr<Prototype> createPrototype(PrototypeType type) {
    switch (type) {
    case PrototypeType::One: {
      auto firstConcrete = std::make_unique<ConcretePrototypeOne>();
      firstConcrete->internalState = 42;
      return firstConcrete;
    }
    case PrototypeType::Two:
      return std::make_unique<ConcretePrototypeTwo>();
    default:
      throw std::runtime_error("Prototype type not implemented");
    }
  }
};

int main() {
  std::unique_ptr<Prototype> prototypeA =
      PrototypeFactory::createPrototype(PrototypeType::One);
  std::unique_ptr<Prototype> prototypeB =
      PrototypeFactory::createPrototype(PrototypeType::Two);

  auto prototypeC = prototypeA->clone();
  auto prototypeD = prototypeB->clone();
  auto prototypeX = prototypeC->clone();

  prototypeC->doOtherThings();
  prototypeD->doOtherThings();
  prototypeX->doOtherThings();

  prototypeD->printInternalState();
  prototypeX->printInternalState();
}

// Ok I am doing something else than cloning myself
// Just copying myself is too easy. I also pretend to be my daddy
// Ok I am doing something else than cloning myself
// 0
// 42
