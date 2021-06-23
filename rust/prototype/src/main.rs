pub trait Prototype {
    fn clone(&self) -> Box<dyn Prototype>;
    fn print_internal_state(&self);
}

#[derive(Default)]
struct ConcretePrototypeOne {
    state: i32,
}

#[derive(Default)]
struct ConcretePrototypeTwo {
    state: String,
}

impl Prototype for ConcretePrototypeOne {
    fn clone(&self) -> Box<dyn Prototype> {
        let new_concrete = ConcretePrototypeOne { state: self.state };
        Box::new(new_concrete)
    }

    fn print_internal_state(&self) {
        println!("State: {}", self.state);
    }
}

impl Prototype for ConcretePrototypeTwo {
    fn clone(&self) -> Box<dyn Prototype> {
        let new_concrete = ConcretePrototypeTwo {
            state: self.state.clone(),
        };
        Box::new(new_concrete)
    }
    fn print_internal_state(&self) {
        println!("State: {}", self.state);
    }
}

enum PrototypeClass {
    WithInt,
    WithString,
}

struct PrototypeFactory;
impl PrototypeFactory {
    fn create_prototype(class: PrototypeClass) -> Box<dyn Prototype> {
        match class {
            PrototypeClass::WithInt => Box::new(ConcretePrototypeOne { state: 42 }),
            PrototypeClass::WithString => Box::new(ConcretePrototypeTwo {
                state: "Help! I am copied!".into(),
            }),
        }
    }
}
fn main() {
    let prototype_a = PrototypeFactory::create_prototype(PrototypeClass::WithInt);
    let prototype_b = PrototypeFactory::create_prototype(PrototypeClass::WithString);

    let prototype_c = prototype_a.clone();
    let prototype_d = prototype_b.clone();
    let prototype_x = prototype_c.clone();

    prototype_a.print_internal_state();
    prototype_b.print_internal_state();
    prototype_c.print_internal_state();
    prototype_d.print_internal_state();
    prototype_x.print_internal_state();
}

// State: 42
// State: Help! I am copied!
// State: 42
// State: Help! I am copied!
// State: 42
