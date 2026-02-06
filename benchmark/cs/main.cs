using System;
using System.Diagnostics;

class Program
{
    static void Main(string[] args)
    {
        long n = (args.Length > 0) ? long.Parse(args[0]) : 100000000L;

        // START
        Stopwatch duration = Stopwatch.StartNew();
        long total = 0;
        for (long i = 0; i < n; i++)
        {
            total += i;
        }
        duration.Stop();
        // END

        double seconds = duration.Elapsed.TotalSeconds;
        double milliseconds = seconds * 1000;
        double microseconds = seconds * 1000000;

        double result;
        string unit;
        if (seconds >= 1.0)
        {
            result = seconds;
            unit = "s";
        }
        else if (milliseconds >= 1.0)
        {
            result = milliseconds;
            unit = "ms";
        }
        else
        {
            result = microseconds;
            unit = "us";
        }

       Console.WriteLine("C#: {0:F1} {1} ({2})", result, unit, total);
    }
}
