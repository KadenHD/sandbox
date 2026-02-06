function main(): void {
    const n: number = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 100_000_000;

    /* START */
    const start: number = performance.now();
    let total: bigint = BigInt(0);
    for (let i: bigint = BigInt(0); i < n; i++) {
        total += i;
    }
    const duration: number = performance.now() - start;
    /* END */

    const seconds: number = duration / 1_000;
    const milliseconds: number = seconds * 1_000;
    const microseconds: number = seconds * 1_000_000;

    let result: number, unit: string;
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

    console.log(`TypeScript: ${result.toFixed(1)} ${unit} (${total})`);
}

main();
