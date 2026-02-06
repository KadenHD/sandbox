import time
import sys

def main():
    n: int = int(sys.argv[1]) if len(sys.argv) > 1 else 100_000_000

    """ START """
    start: float = time.perf_counter()
    total: int = 0
    for i in range(n):
        total += i
    duration: float = time.perf_counter() - start
    """  END  """

    seconds: float = duration
    milliseconds: float = seconds * 1_000.0
    microseconds: float = seconds * 1_000_000.0

    if seconds >= 1.0:
        result, unit = seconds, "s"
    elif milliseconds >= 1.0:
        result, unit = milliseconds, "ms"
    else:
        result, unit = microseconds, "us"

    print(f"Python: {result:.1f} {unit} ({total})")

if __name__ == "__main__":
    main()
