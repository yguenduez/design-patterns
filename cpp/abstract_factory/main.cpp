#include <iostream>
#include <memory>

class ProductAInterface {
public:
  virtual void tellWhoIam() const = 0;
  virtual ~ProductAInterface() {}
};

class ProductBInterface {
public:
  virtual void tellOtherStuff() const = 0;
  virtual ~ProductBInterface() {}
};

class AbstractFactory {
public:
  virtual std::unique_ptr<ProductAInterface> createProductA() const = 0;
  virtual std::unique_ptr<ProductBInterface> createProductB() const = 0;

  virtual ~AbstractFactory() {}
};

class ConcreteProductA1 : public ProductAInterface {
public:
  void tellWhoIam() const override { std::cout << "A1" << std::endl; }
};
class ConcreteProductA2 : public ProductAInterface {
public:
  void tellWhoIam() const override { std::cout << "A2" << std::endl; }
};

class ConcreteProductB1 : public ProductBInterface {
public:
  void tellOtherStuff() const override { std::cout << "Me be B1" << std::endl; }
};
class ConcreteProductB2 : public ProductBInterface {
public:
  void tellOtherStuff() const override { std::cout << "Me be B2" << std::endl; }
};

class FirstConcreteFactory : public AbstractFactory {
public:
  std::unique_ptr<ProductAInterface> createProductA() const override {
    return std::make_unique<ConcreteProductA1>();
  }
  std::unique_ptr<ProductBInterface> createProductB() const override {
    return std::make_unique<ConcreteProductB1>();
  }
};

class SecondConcreteFactory : public AbstractFactory {
public:
  std::unique_ptr<ProductAInterface> createProductA() const override {
    return std::make_unique<ConcreteProductA2>();
  }

  std::unique_ptr<ProductBInterface> createProductB() const override {
    return std::make_unique<ConcreteProductB2>();
  }
};

void client(AbstractFactory const &factory) {
  auto productA = factory.createProductA();
  auto productB = factory.createProductB();

  productA->tellWhoIam();
  productB->tellOtherStuff();
};

int main() {
  std::unique_ptr<AbstractFactory> firstFactory =
      std::make_unique<FirstConcreteFactory>();

  std::unique_ptr<AbstractFactory> secondFactory =
      std::make_unique<SecondConcreteFactory>();

  client(*firstFactory);
  client(*secondFactory);
}
