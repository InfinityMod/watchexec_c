#!/bin/sh
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:.
cd "${0%/*}"
./watch_test 
