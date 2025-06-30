## Notes

A common example given is that the module structure is very similar to a file hierarchy

$ tree .
.
├── my
│   ├── inaccessible.rs
│   └── nested.rs
├── my.rs
└── split.rs


Tree(.) being the root and the other modules/files being put in their own section.
Other files can call each other as long as you include them in the main file, main.rs or lib.rs
using 

mod inaccessible;

for example
