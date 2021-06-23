pub trait ProductA {
    fn tell_who_i_am(&self);
}

pub trait ProductB {
    fn tell_other_stuff(&self);
}

pub trait AbstractFactory {
    fn create_product_a(&self) -> Box<dyn ProductA>;
    fn create_product_b(&self) -> Box<dyn ProductB>;
}

struct FirstConcreteFactory;
struct SecondConecreteFactory;
struct ConcreteProductA1;
struct ConcreteProductA2;
struct ConcreteProductB1;
struct ConcreteProductB2;

impl ProductA for ConcreteProductA1 {
    fn tell_who_i_am(&self) {
        println!("I am A1");
    }
}

impl ProductA for ConcreteProductA2 {
    fn tell_who_i_am(&self) {
        println!("I am A2");
    }
}

impl ProductB for ConcreteProductB1 {
    fn tell_other_stuff(&self) {
        println!("I am B1");
    }
}

impl ProductB for ConcreteProductB2 {
    fn tell_other_stuff(&self) {
        println!("I am B2");
    }
}

impl AbstractFactory for FirstConcreteFactory {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(ConcreteProductA1 {})
    }

    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(ConcreteProductB1 {})
    }
}

impl AbstractFactory for SecondConecreteFactory {
    fn create_product_a(&self) -> Box<dyn ProductA> {
        Box::new(ConcreteProductA2 {})
    }
    fn create_product_b(&self) -> Box<dyn ProductB> {
        Box::new(ConcreteProductB2 {})
    }
}

pub fn client_code(factory: &dyn AbstractFactory) {
    let product_a = factory.create_product_a();
    let product_b = factory.create_product_b();

    product_a.tell_who_i_am();
    product_b.tell_other_stuff();
}

fn main() {
    let first_factory = FirstConcreteFactory {};
    let second_factory = SecondConecreteFactory {};

    client_code(&first_factory);
    client_code(&second_factory);
}
