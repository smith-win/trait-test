Playing around with traits.  Especially seeing how I can use them to
help implement a tpye-safe binding for binding params to OCI.

 - Using traits to hepl map Rust types to OCI types (num, int, chr)
 - Allowing rust types to support more than one OCI type in parameter binds
 - Generating traits from macros
 - Using macros to then apply comon functions these traits, avoiding dynamic dispatch

