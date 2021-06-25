#include <cstddef>
#include <iostream>
#include <memory>
#include <vector>

struct Leg {};
struct Head {};
struct Arm {};

class HumanBody {
private:
  std::vector<Leg> legs;
  std::vector<Head> heads;
  std::vector<Arm> arms;

public:
  void addArm(Arm &&arm) { arms.push_back(std::move(arm)); }
  void addLef(Leg &&leg) { legs.push_back(std::move(leg)); }
  void addHead(Head &&head) { heads.push_back(std::move(head)); }

  size_t legsCount() const { return legs.size(); };
  size_t headsCount() const { return heads.size(); };
  size_t armsCount() const { return arms.size(); };
};

class Builder {
public:
  virtual Builder &buildHumanCorpus() = 0;
  virtual Builder &buildArm() = 0;
  virtual Builder &buildLeg() = 0;
  virtual Builder &buildHead() = 0;

  virtual HumanBody const &getBody() = 0;

  virtual ~Builder() = default;
};

class Director {
public:
  virtual void createHuman(Builder &builder) = 0;
  virtual ~Director() = default;
};

class GodDirector : Director {
public:
  void createHuman(Builder &builder) {
    builder.buildHumanCorpus()
        .buildLeg()
        .buildLeg()
        .buildHead()
        .buildArm()
        .buildArm();
  }
};

class DevilDirector : Director {
public:
  void createHuman(Builder &builder) {
    builder.buildHumanCorpus()
        .buildLeg()
        .buildLeg()
        .buildLeg()
        .buildHead()
        .buildHead()
        .buildArm()
        .buildArm();
  }
};

struct BodyBuilder : Builder {
public:
  Builder &buildHumanCorpus() override {
    if (!body) {
      body = std::make_unique<HumanBody>();
    }
    return *this;
  }

  Builder &buildArm() override {
    body->addArm(Arm{});
    return *this;
  };
  Builder &buildLeg() override {
    body->addLef(Leg{});
    return *this;
  };
  Builder &buildHead() override {
    body->addHead(Head{});
    return *this;
  };

  HumanBody const &getBody() override { return *body; }

private:
  std::unique_ptr<HumanBody> body;
};

void analyzeBody(HumanBody const &body) {
  std::printf("The body has %zu arms, %zu heads and %zu legs\n",
              body.armsCount(), body.headsCount(), body.legsCount());
}

int main() {
  std::unique_ptr<Builder> builder = std::make_unique<BodyBuilder>();
  GodDirector director{};
  director.createHuman(*builder);

  auto body = builder->getBody();
  analyzeBody(body);
  // The body has 2 arms, 1 heads and 2 legs

  std::unique_ptr<Builder> otherBuilder = std::make_unique<BodyBuilder>();
  DevilDirector otherDirector{};
  otherDirector.createHuman(*otherBuilder);

  auto otherBody = otherBuilder->getBody();
  analyzeBody(otherBody);
  // The body has 2 arms, 2 heads and 3 legs
}
