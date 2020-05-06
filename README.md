### Simple calculator that supports variables, functions and matrix multiplication

```
make && ./ComputorV2

> 2 + 2
Parsed: (2 + 2)
Result: 4
> 2 + 3^4 * 5
Parsed: (2 + ((3 ^ 4) * 5))
Result: 407
> f(x, y) = x + 2 * y
Parsed: (function)
Result: (x + (2 * y))
> f(1, 5)
Parsed: (f)(...)
Result: 11
> m = [[1, 2]; [3, 4]]
Parsed: m = [[1.0, 2.0], [3.0, 4.0]]
Result: [[1.0, 2.0], [3.0, 4.0]]
> m * 2 + 1
Parsed: ((m * 2) + 1)
Result: [[3.0, 5.0], [7.0, 9.0]]
> n = [[7, 7]]
Parsed: n = [[7.0, 7.0]]
Result: [[7.0, 7.0]]
> m ** n
Parsed: (m ** n)
Result: [[21.0], [49.0]]
> g(x) = x + a
Parsed: (function)
Result: (x + a)
> a = 5
Parsed: a = 5
Result: 5
> g(6)
Parsed: (g)(...)
Result: 11
```
