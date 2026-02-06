#include <iostream>
#include <chrono>
#include <cstdlib>
#include <iomanip>

int main(int argc, char* argv[]) {
    long long n = (argc > 1) ? std::atoll(argv[1]) : 100000000LL;

    /* START */
    auto start = std::chrono::high_resolution_clock::now();
    long long total = 0;
    for (long long i = 0; i < n; i++) {
        total += i;
    }
    auto end = std::chrono::high_resolution_clock::now();
    /*  END  */

    std::chrono::duration<double> duration = end - start;

    double seconds = duration.count();
    double milliseconds = seconds * 1000.0;
    double microseconds = seconds * 1000000.0;

    double result;
    std::string unit;
    if (seconds >= 1.0) {
        result = seconds;
        unit = "s";
    } else if (milliseconds >= 1.0) {
        result = milliseconds;
        unit = "ms";
    } else {
        result = microseconds;
        unit = "us";
    }

    std::cout << "C++: " << std::fixed << std::setprecision(1) << result << " " << unit << " (" << total << ")" << "\n";
}
