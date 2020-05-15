add(x, y) = x + y
mul(x, y) = x * y
pow(x, y) = x ^ y

@enum Par val var
@enum Fun add mul pow nop

f(x) = add(x...)

println(f([1,2]))
