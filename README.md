Third party crate to add C(C++)-binding support to _watchexec_. 

_Watchexec_ _v.1.9.3_ is required as some parts needed to be aligned for the implementation.

The crate exports the watchexec program including the c-bindings into a dynamic library (linux: _libwatchexec_c.so_).

A sample C++ application is appended to the _"./test"_ folder. For a short testing (under linux) just run:

**make test**  
**make test_run**

Change some files at _"./test/to_watch"_. The file changes should be displayed in the executed console. Compilation requires _g++_.