package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

func main() {
	n := 100_000_000
	if len(os.Args) > 1 {
		if v, err := strconv.Atoi(os.Args[1]); err == nil {
			n = v
		}
	}

	// START //
	start := time.Now()
	total := 0
	for i := 0; i < n; i++ {
		total += i
	}
	duration := time.Since(start)
	//  END  //

	seconds := duration.Seconds()
	milliseconds := seconds * 1_000.0
	microseconds := seconds * 1_000_000.0

	var result float64
	var unit string

	if seconds >= 1.0 {
		result = seconds
		unit = "s"
	} else if milliseconds >= 1.0 {
		result = milliseconds
		unit = "ms"
	} else {
		result = microseconds
		unit = "us"
	}

	fmt.Printf("Go: %.1f %s (%d)\n", result, unit, total)
}
