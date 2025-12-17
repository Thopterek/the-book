# Declaring modules #
in any file other than crate root you can declare new modules,
compiler for module mod garden; would look into
* Inline, within curly brackets that replace semicolon with mod garden
* in the file src/garden.rs
* in the file src/garden/mod.rs

# Submodules #
same rule you need to omit the crate root to declare submodule
the compilers look for the submodule code within directory name for parent
* Inline, directly following mod vegetable, within curly brackets
* In the file src/garden/vegetables.rs
* In the file src/garden/vegetables/mod.rs

# Path to code in modules #
once a module is part of your crate you can refer to it in the same crate
as long as privacy rules allow for it, example of Asparagus as type in garden veg..
* found at -> crate::garden::vegetables::Asparagus

# Private vs Public #
code is private by default, to make it public use keyword pub mod (replacing just mod)
taking part and making item public also needs to add the pub keyword before declaring

# The use keyword #
creating a shortuc to items to shorten path (namespace like?)
* you can refer to -> crate::garden::vegetables::Asparagus
* but if you used use ^ you just need to write Asparagus

# Example #
crate backyard
