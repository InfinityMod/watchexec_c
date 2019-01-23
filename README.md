Third party crate to add C(C++)-binding support to watchexec. 

Watchexec v. 1.9.3 is required as some parts needed to be aligned for the implementation.
The crate exports the watchexec program including the c-bindings into a dynamic library (linux: libwatchexec_c.so).
A sample C++ application is integrated at the "test" folder. For a short testing (under linux) just run:

make test  
make test_run

and change some files at ./test/to_watch. The file changes should be displayed. Compilation requires g++.
