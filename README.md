Apple ][ Kit Macros
====================

These are the procedural macros for [a2kit](https://github.com/dfgordon/a2kit), which are required to reside in their own crate.

This is the outer crate, containing the traits we want to derive.  The inner crate has the actual derivations.

As of this writing there is only one trait involved, `DiskStruct`.