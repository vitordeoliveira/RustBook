# More About Cargo and Crates.io

<!--toc:start-->

- [More About Cargo and Crates.io](#more-about-cargo-and-cratesio)
  - [Deprecating Versions from Crates.io with cargo yank](#deprecating-versions-from-cratesio-with-cargo-yank)
  <!--toc:end-->

So far we’ve used only the most basic features of Cargo to build, run, and test
our code, but it can do a lot more. In this chapter, we’ll discuss some of its
other, more advanced features to show you how to do the following:

Customize your build through release profiles
Publish libraries on crates.io
Organize large projects with workspaces
Install binaries from crates.io
Extend Cargo using custom commands

More in [cargo docs](https://doc.rust-lang.org/cargo/)

## Deprecating Versions from Crates.io with cargo yank

Although you can’t remove previous versions of a crate, you can prevent any
future projects from adding them as a new dependency. This is useful when a
crate version is broken for one reason or another. In such situations, Cargo
supports yanking a crate version.

Yanking a version prevents new projects from depending on that version while
allowing all existing projects that depend on it to continue. Essentially, a
yank means that all projects with a Cargo.lock will not break, and any future
Cargo.lock files generated will not use the yanked version.

Be careful, because a publish is permanent. The version can never be
overwritten, and the code cannot be deleted. One major goal of crates.io is to
act as a permanent archive of code so that builds of all projects that depend
on crates from crates.io will continue to work. Allowing version deletions
would make fulfilling that goal impossible. However, there is no limit to the
number of crate versions you can publish.

```bash
cargo publish
```

To yank a version of a crate, in the directory of the crate that you’ve
previously published, run cargo yank and specify which version you want to
yank.

```#!/bin/bash
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo
```

> Note that if you want Rust to treat undocumented code as an error, you can
> add the following statement at the root of your library:

```rust-lang
deny(missing_docs)]
```
