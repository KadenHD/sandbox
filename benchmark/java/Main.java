public class Main {
    public static void main(String[] args) {
        long n = args.length > 0 ? Long.parseLong(args[0]) : 100_000_000L;

        /* START */
        long start = System.nanoTime();
        long total = 0;
        for (long i = 0; i < n; i++) {
            total += i;
        }
        long duration = System.nanoTime() - start;
        /*  END  */

        double seconds = duration / 1_000_000_000.0;
        double milliseconds = seconds * 1_000.0;
        double microseconds = seconds * 1_000_000.0;

        double result;
        String unit;
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

        System.out.printf("Java: %.1f %s (%d)%n", result, unit, total);
    }
}
