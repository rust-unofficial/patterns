# Design principles

## A brief overview over common design principles

---

## [SOLID](https://en.wikipedia.org/wiki/SOLID)

- [Single Responsibility Principle (SRP)](https://en.wikipedia.org/wiki/Single-responsibility_principle):
  A class should only have a single responsibility, that is, only changes to
  one part of the software's specification should be able to affect the
  specification of the class.
- [Open/Closed Principle (OCP)](https://en.wikipedia.org/wiki/Open%E2%80%93closed_principle):
  "Software entities ... should be open for extension, but closed for
  modification."
- [Liskov Substitution Principle (LSP)](https://en.wikipedia.org/wiki/Liskov_substitution_principle):
  "Objects in a program should be replaceable with instances of their subtypes
  without altering the correctness of that program."
- [Interface Segregation Principle (ISP)](https://en.wikipedia.org/wiki/Interface_segregation_principle):
  "Many client-specific interfaces are better than one general-purpose
  interface."
- [Dependency Inversion Principle (DIP)](https://en.wikipedia.org/wiki/Dependency_inversion_principle):
  One should "depend upon abstractions, [not] concretions."

## [DRY (Don’t Repeat Yourself)](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself)

"Every piece of knowledge must have a single, unambiguous, authoritative
representation within a system"

## [KISS principle](https://en.wikipedia.org/wiki/KISS_principle)

most systems work best if they are kept simple rather than made complicated;
therefore, simplicity should be a key goal in design, and unnecessary
complexity should be avoided

## [Law of Demeter (LoD)](https://en.wikipedia.org/wiki/Law_of_Demeter)

a given object should assume as little as possible about the structure or
properties of anything else (including its subcomponents), in accordance with
the principle of "information hiding"

## [Design by contract (DbC)](https://en.wikipedia.org/wiki/Design_by_contract)

software designers should define formal, precise and verifiable interface
specifications for software components, which extend the ordinary definition of
abstract data types with preconditions, postconditions and invariants

## [Encapsulation](https://en.wikipedia.org/wiki/Encapsulation_(computer_programming))

bundling of data with the methods that operate on that data, or the restricting
of direct access to some of an object's components. Encapsulation is used to
hide the values or state of a structured data object inside a class, preventing
unauthorized parties' direct access to them.

## [Command-Query-Separation(CQS)](https://en.wikipedia.org/wiki/Command%E2%80%93query_separation)

“Functions should not produce abstract side effects...only commands
(procedures) will be permitted to produce side effects.” - Bertrand Meyer:
Object-Oriented Software Construction

## [Principle of least astonishment (POLA)](https://en.wikipedia.org/wiki/Principle_of_least_astonishment)

a component of a system should behave in a way that most users will expect it
to behave. The behavior should not astonish or surprise users

## Linguistic-Modular-Units

“Modules must correspond to syntactic units in the language used.” - Bertrand
Meyer: Object-Oriented Software Construction

## Self-Documentation

“The designer of a module should strive to make all information about the
module part of the module itself.” - Bertrand Meyer: Object-Oriented Software
Construction

## Uniform-Access

“All services offered by a module should be available through a uniform
notation, which does not betray whether they are implemented through storage or
through computation.” - Bertrand Meyer: Object-Oriented Software Construction

## Single-Choice

“Whenever a software system must support a set of alternatives, one and only
one module in the system should know their exhaustive list.” - Bertrand Meyer:
Object-Oriented Software Construction

## Persistence-Closure

“Whenever a storage mechanism stores an object, it must store with it the
dependents of that object. Whenever a retrieval mechanism retrieves a
previously stored object, it must also retrieve any dependent of that object
that has not yet been retrieved.” - Bertrand Meyer: Object-Oriented Software
Construction
