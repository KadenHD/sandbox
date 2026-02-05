function main() {
    const n = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 100_000_000;

    /* START */
    const start = performance.now();
    let _total = 0;
    for (let i = 0; i < n; i++) {
        _total += i;
    }
    const duration = performance.now(); - start;
    /*  END  */

    const seconds = duration / 1_000;
    const milliseconds = seconds * 1_000;
    const microseconds = seconds * 1_000_000;

    let result, unit;
    if (seconds >= 1.0) {
        result = seconds;
        unit = "s";
    } else if (milliseconds >= 1.0) {
        result = milliseconds;
        unit = "ms";
    } else {
        result = microseconds;
        unit = "µs";
    }

    console.log(`JavaScript: ${result.toFixed(1)} ${unit}`);
}

main();
