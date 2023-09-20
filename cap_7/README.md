## Managing Growing Projects with Packages, Crates, and Modules

The programs we’ve written so far have been in one module in one file. As a project grows, you should organize code by splitting it into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. This chapter covers all these techniques. For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces, which we’ll cover in the “Cargo Workspaces” section in Chapter 14.

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of path
Paths: A way of naming an item, such as a struct, function, or module