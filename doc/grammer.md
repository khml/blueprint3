sum = mul { “+” mul | “-“ mul }
mul = priority { “*” priority | “/“ priority | “%” priority }
priority = number | “(“ sum “)”