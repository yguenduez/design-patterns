pub trait ProductInterface {
    fn do_something(&self);
}

pub enum ProductType {
    A,
    B,
    C,
}

struct ProductA;
struct ProductB;
struct ProductC;

impl ProductInterface for ProductA {
    fn do_something(&self) {
        println!("I am A");
    }
}

impl ProductInterface for ProductB {
    fn do_something(&self) {
        println!("I am B");
    }
}

impl ProductInterface for ProductC {
    fn do_something(&self) {
        println!("I am C");
    }
}

struct Factory;
impl Factory {
    fn create_product(product_type: ProductType) -> Box<dyn ProductInterface> {
        match product_type {
            ProductType::A => Box::new(ProductA),
            ProductType::B => Box::new(ProductB),
            ProductType::C => Box::new(ProductC),
        }
    }
}

pub fn client_is_not_interested_in_concrete_type(product: &dyn ProductInterface) {
    product.do_something();
}

fn main() {
    let product_a = Factory::create_product(ProductType::A);
    let product_b = Factory::create_product(ProductType::B);
    let product_c = Factory::create_product(ProductType::C);

    let products: Vec<&dyn ProductInterface> = vec![&*product_a, &*product_b, &*product_c];
    let mut product_iterator = products.iter();
    while let Some(product) = product_iterator.next() {
        client_is_not_interested_in_concrete_type(*product)
    }
}
