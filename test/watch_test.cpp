#include <stdio.h>
#include <inttypes.h>
#include <time.h>

extern "C" {
    #include "watchexec_c.h"
}

void echo(uint32_t cnt){
    printf("Values: %d\n", cnt);
}

void echo_results(uint8_t state, DynArray results){
    struct tm  ts;
    printf("Changes: %d\n", results.length);
    for (size_t i=0; i<results.length; i++){

        int64_t* unix_time = &(*results.array)[i].time;
        
        char time[80];
        ts = *localtime(unix_time);
        strftime(time, sizeof(time), "%a %Y-%m-%d %H:%M:%S %Z", &ts);
        
        char* path = (*results.array)[i].path;
        char* op = (*results.array)[i].op;
        uint32_t cookie = (*results.array)[i].cookie;

        printf("%s\n%s\n%s\n%u\n\n", time, op, path, cookie);
    }
}

int main(){
    //count_substrings("banana", "na", echo);
    run_watcher("watchexec --no-vcs-ignore -p -w ./to_watch null", echo_results);
    return 0;
}
