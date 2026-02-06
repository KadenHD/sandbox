#include <stdio.h>
#include <stdlib.h>

#if defined(_WIN32)
    #include <windows.h>
#else
    #define _POSIX_C_SOURCE 199309L
    #include <time.h>
#endif

static double record(void) {
    #if defined(_WIN32)
        LARGE_INTEGER freq, counter;
        QueryPerformanceFrequency(&freq);
        QueryPerformanceCounter(&counter);
        return (double)counter.QuadPart / freq.QuadPart;
    #else
        struct timespec ts;
        clock_gettime(CLOCK_MONOTONIC, &ts);
        return ts.tv_sec + ts.tv_nsec * 1e-9;
    #endif
}

int main(int argc, char *argv[]) {
    long long n = (argc > 1) ? atoll(argv[1]) : 100000000LL;

    /* START */
    double start = record();
    long long total = 0;
    for (long long i = 0; i < n; i++) {
        total += i;
    }
    double duration = record() - start;
    /*  END  */

    double seconds = duration;
    double milliseconds = seconds * 1000.0;
    double microseconds = seconds * 1000000.0;

    double result;
    char unit[2];
    if (seconds >= 1.0) {
        result = seconds;
        strcpy(unit, "s");
    }
    else if (milliseconds >= 1.0) {
        result = milliseconds;
        strcpy(unit, "ms");
    }
    else {
        result = microseconds;
        strcpy(unit, "us");
    }

    printf("C: %.1f %s (%lld)\n", result, unit, total);
}
