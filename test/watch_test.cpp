#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <time.h>
#include <chrono>
#include "./date.h"

extern "C" {
    #include "watchexec_c.h"
}

std::chrono::time_point<std::chrono::system_clock> int_to_timestamp(int64_t sec, uint32_t micro){
    std::chrono::seconds dur_sec(sec);
    std::chrono::microseconds dur_micro(micro);
    std::chrono::time_point<std::chrono::system_clock> dt(dur_sec+dur_micro);
    return dt;
}


void echo(uint32_t cnt){
    printf("Values: %d\n", cnt);
}

void echo_results(uint8_t state, DynArray _results){
    struct tm  ts;
    size_t n;
    DynArray::PathOp *results = new DynArray::PathOp[_results.length];
    size_t size = _results.length;

    for(n=0; n<size; n++){
        results[n] = *(&(*_results.array)[n]);
    }

    printf("Changes: %d\n", size);

    for (size_t i=0; i<size; i++){
        int64_t* unix_time = &results[i].time;
        uint32_t* unix_time_micro = &results[i].time_micro;
        std::chrono::time_point<std::chrono::system_clock> ts = int_to_timestamp(*unix_time, *unix_time_micro);
        
        char time[80];
        strcpy(time, date::format("%D %T %Z\n", date::floor<std::chrono::milliseconds>(ts)).c_str());

        char* path = (results[i]).path;
        char* op = (results[i]).op;
        uint32_t cookie = (results[i]).cookie;
        
        printf("%s\n%s\n%s\n%u\n\n", time, op, path, cookie);
    }
}

int main(){
    //count_substrings("banana", "na", echo);
    run_watcher("watchexec --no-vcs-ignore -p -w ./to_watch null", echo_results);
    return 0;
}
