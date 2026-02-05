<?php

function main() {
    $n = (int)$GLOBALS['argv'][1] ?? 100_000_000;

    /** START **/
    $start = hrtime(true);
    $total = 0;
    for ($i = 0; $i < $n; $i++) {
        $total += $i;
    }
    $duration = hrtime(true) - $start;
    /**  END  **/

    $seconds = $duration / 1_000_000_000;
    $milliseconds = $seconds * 1_000.0;
    $microseconds = $seconds * 1_000_000.0;

    if ($seconds >= 1.0) {
        $result = $seconds;
        $unit = "s";
    } elseif ($milliseconds >= 1.0) {
        $result = $milliseconds;
        $unit = "ms";
    } else {
        $result = $microseconds;
        $unit = "µs";
    }

    printf("PHP: %.1f %s\n", $result, $unit);
}

main();
