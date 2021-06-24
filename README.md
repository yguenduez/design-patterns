# Design Patterns

The repository is only for repeating and practising object-oriented design patterns with minimal examples.
The reference is "Design Patterns: Elements of Reusable Object-Oriented Software" by the Gang of Four.

## The following patterns are implemented

| Pattern                  | C++ | Rust  |
| ------------------------ | --- | ----- |
| Abstract Factory Pattern | [X] | [X]   |
| Factory Method           | [X] | [X]   |
| Builder Pattern          | [X] | [X]   |
| Prototype                | [X] | [X]   |
| Singleton                | [ ] | [ ]   |
| ------------------------ | --- | ----- |
| Adapter Pattern          | [ ] | [ ]   |
| Bridge Pattern           | [ ] | [ ]   |
| Composite                | [ ] | [ ]   |
| Decorator                | [ ] | [ ]   |
| Flyweight                | [ ] | [ ]   |
| Facade                   | [ ] | [ ]   |
| Proxy                    | [ ] | [ ]   |
| ------------------------ | --- | ----- |
| Command Pattern          | [ ] | [ ]   |
| Chain of Responsibility  | [ ] | [ ]   |
| Interpreter              | [ ] | [ ]   |
| Interator                | [ ] | [ ]   |
| Mediator                 | [ ] | [ ]   |
| Memento                  | [ ] | [ ]   |
| Observer                 | [ ] | [ ]   |
| State Pattern            | [ ] | [ ]   |
| Strategy Pattern         | [ ] | [ ]   |
| Visitor                  | [ ] | [ ]   |
| Template                 | [ ] | [ ]   |

## Building and Testing

In order to build and/or run the samples, just do the following:

- cpp:

  ```bash
  cd <subfolder> && mkdir build && cd build
  cmake ..
  make
  ./<executable>
  ```

- rust:
  
  ```bash
  cd <subfolder>
  cargo build/run
  ```