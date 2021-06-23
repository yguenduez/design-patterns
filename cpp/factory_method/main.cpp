#include <iostream>
#include <memory>

class ProductInterface {
public:
  virtual void do_something() const = 0;

  virtual ~ProductInterface() = default;
};

class ProductCreatorInterface {
public:
  virtual std::unique_ptr<ProductInterface> createProduct() const = 0;

  virtual ~ProductCreatorInterface() = default;
};

template <typename ConcreteProductType>
class ConcreteCreator : public ProductCreatorInterface {
public:
  std::unique_ptr<ProductInterface> createProduct() const override;
};

template <typename ConcreteProductType>
std::unique_ptr<ProductInterface>
ConcreteCreator<ConcreteProductType>::createProduct() const {
  return std::make_unique<ConcreteProductType>();
}

struct ProductA : ProductInterface {
  void do_something() const override { std::cout << "I am A" << std::endl; }
};

struct ProductB : ProductInterface {
  void do_something() const override { std::cout << "I am B" << std::endl; }
};

struct ProductC : ProductInterface {
  void do_something() const override { std::cout << "I am C" << std::endl; }
};

void clientCode(ProductCreatorInterface const &creator) {
  // we also could have used simply auto here, i.e:
  // auto product = creator.createProduct()
  std::unique_ptr<ProductInterface> product = creator.createProduct();
  product->do_something();
}

int main() {
  ProductCreatorInterface const &creatorA = ConcreteCreator<ProductA>();
  ProductCreatorInterface const &creatorB = ConcreteCreator<ProductB>();
  ProductCreatorInterface const &creatorC = ConcreteCreator<ProductC>();

  clientCode(creatorA);
  clientCode(creatorB);
  clientCode(creatorC);
}
