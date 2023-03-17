
/*
    variable, cons, list, object, object-list, call pattern, next pattern, or pattern, if pattern, and pattern, xor pattern
    at pattern, tuple pattern

    * x
    * (a, b)
    * [], [x], [x, y, ...]
    * {[x; y; z]}
    * {[* x; ? y;]}  // ?
    * x(y, z, w) 
    * {[x]}
    * { x; y; z; }
    * { !; x(y, !, w); }
    * { w(a, (x @ !).and(a:b).and(c:d) ) }  // b and d are some patterns with some known list of anon-struct return value, which is dropped into a and c
    * $x(a) // call pattern 'x' and match against the returned value


*/