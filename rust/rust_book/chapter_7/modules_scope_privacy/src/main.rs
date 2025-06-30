// There are some rules that reference how rust uses modules, paths and the use and pub keyword
// to control scope, privacy and make sure to limit what a module seens in the other modules.
//
// The first rule is that the compiler looks at the create root for code to compile.
// You can declare new modules with the keyword mod. e.g mod garden
// if you declare a mod, the rust compiler will begin to look for it in 3 parts:
// 1. in the curly brackets after mod garden. e.g mod garden{}
// 2. in the file src/garden.rs
// 3. in the file src/garden/mod.rs
//
// You can also declare submodules, modules in other modules. For example declaring
// mod vegetables in the file src/garden.rs. In that case the rust compiler will
// look for that module in:
// 1. in the curly brakets after mod vegetables. e.g mod vegetables{}
// 2. in the file src/garden/vegetables.rs
// 3. in the file src/garden/vegetables/mod.rs
//
// Once a module is part of the crate, you can refer to that module from anywhere in the same
// crate. E.g if asparagus is a type in garden vetables module, then the module is found in
// crate::garden::vegetables::Asparagus.
//
// Code and modules are automatically made private and you need to use the keyword pub to make it public
// to anything not in the same module. E.g pub mod to make a module public and just pub to make
// code public
//
// Use is a way to create a shortcut, instead of writing crate::garden::vegetables::Asparagus
// everytime, you can use crate::garden::vegetables::Asparagus; and then you only need to write
// Asparagus to use that type in the scope.

mod garden;

fn main() {
    let plant = crate::garden::vegetables::Asparagus {};
    println!("I'm growing {plant:?}!");
}
