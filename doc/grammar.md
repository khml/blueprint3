statement = sum | assignment
assignment = "let" identifier "=" sum
sum = mul { “+” mul | “-“ mul }*
mul = priority { “*” priority | “/“ priority | “%” priority }*
priority = number | “(“ sum “)”