def main
  n = ARGV.length > 0 ? ARGV[0].to_i : 100_000_000

  # START #
  start = Process.clock_gettime(Process::CLOCK_MONOTONIC)
  total = 0
  n.times do |i|
    total += i
  end
  duration = Process.clock_gettime(Process::CLOCK_MONOTONIC) - start
  #  END  #

  seconds = duration
  milliseconds = seconds * 1_000.0
  microseconds = seconds * 1_000_000.0

  if seconds >= 1.0
    result = seconds
    unit = "s"
  elsif milliseconds >= 1.0
    result = milliseconds
    unit = "ms"
  else
    result = microseconds
    unit = "us"
  end

  puts format("Ruby: %.1f %s (%d)", result, unit, total)
end

main
