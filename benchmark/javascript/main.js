function main() {
    const n = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 100_000_000;

    /* START */
    const start = performance.now();
    let total = 0n;
    for (let i = 0n; i < n; i++) {
        total += i;
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
        unit = "us";
    }

    console.log(`JavaScript: ${result.toFixed(1)} ${unit} (${total})`);
}

main();
