# Rather than recreating the structure and code, rules to follow

* module declared in root module [name] -> looks for src/[name].rs
* exposing through creating mod [mod_name] & making pub use crate::[mod_name]::[name]
* making longer connections as per then able to add a [mod_thing] as [name]
* so it can be accessed then through -> [mod_thing]::[function]

!!! Can be moved later on as per making a new sub roots through only declaration !!!
* in file src/[name].rs -> only pub mod [module];
* then src[name]/[module].rs -> The compiler’s rules for which files to check
