# When writing tests, examples, prototypes

* unwrap / expect -> prototype not being sure how to handle
* panic! -> for the tests as to be sure that it handles it gracefully

# You have more information than compiler

* if you can ensure that there is no chance to be Err value -> expect
* should include the documentation in the expect("text here explaining")

# General guidelines for error handling -> panic!

* The bad state, unexpected rather than happening occasionally
* If your code rely on being in the perfect state
* There is no good way to encode the information in the types you use
* When using someone else code thats missbehaving (bad state)

# Custom types for validation

* creating an instance of a type only if its correct (eg. values range)
* function that takes care of creating that value with fn new(var: type) -> custom_type {}
