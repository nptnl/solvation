<img src="./graphic/sigma.png">

<br/><br/>

<img src="./graphic/Solvation.png">

<br/><br/>

I will make a fully capable graphing calculator in Rust and you can't stop me.

As always, I'm not using libraries or any of that. Everything here is homemade :)

# **how to use**

don't even try to run it unless you want to contribute (or fix my terrible formatting), this is niether stable nor useable, nor useful if you did figure it out. I'll change this message if this project becomes any of those three.

math is explained in [./doc/how-to-math.pdf](./doc/how-to-math.pdf)

<img src="./graphic/tex-meme.png" width=300>

**basic syntax:**

Enter most stuff like you would with any normal calculator, except pay attention to spacing. 
Parenthesis dont need spacing, but the operators do. Also, because I'm cool, dot `·` and cross `×` work for multiplication.

```
3 + 4
[Σ] 7
(3 + 4) × 2
[Σ] 14
```

**number parsing:**

Numbers can be interpreted in a variety of ways, but they all are stored as a complex number of two `f64`s. The parsing function for this homemade `Comp` type basically searches for "i" and a "+" or a "-". This means that within one complex number, the spaces surround "+" and "-" can be omitted.

```
3.0
[Σ] 3.0
3+4i
[Σ] 3+4i
3.23-8.0i
[Σ] 3.23-8.0i
9i
[Σ] 0+9i
```

**variables:**

To declare a new variable, use the `=` operator between its name and value. This value can be an expression, and `complete()` will simplify it. Variable and function names are stored not as a `String`, but instead as a `[char; 5]`. This means that while you can still use >5-character names, the code will only store the first 5 characters, meaning it will not be able to distinguish between names `longname2` and `longname1`, and treat them as the same variable or function. The same operator can also be used for re-assignment.

```
x = 4
[Σ] Done
λ = 3 + 2
[Σ] Done
x * λ
[Σ] 20
λ = 6
[Σ] Done
x * λ
[Σ] 24
```

Values in Solvation currently take on one of three types: `Comp` (my homemade complex numbers), `u16` and `bool`. By default, all numbers will be parsed as complex numbers, for consistent arithmetic. To create a `u16`, prefix the value with ``` ` ```. The `++` operator is available to increment `u16` values by one.

```
x = `1
[Σ] Done
x ++
[Σ] Done
x
[Σ] 2
```

**functions:**

To declare a function, use keyword `def`, followed by `name(inp1, inp2, ...)`, then the contents of the function. Solvation will analyze the input names and replace any instances of them with an input token. The commas here are important as they are used to separate inputs instead of spaces.

This example computes the magnitude of a diagonal line set by two inputs.

```
def mag(x, y) (x * x + y * y) ^ 0.5
[Σ] Done
mag(1,2)
[Σ] 2.2360198347089875
mag(1,1)
1.4142134378915125
```

**multitasking:**

You can complete multiple calculations in one line, although only one (the last one) will return. This is useful for variable assignment and reassignment, as these statements will not return values anyways, and rather execute something that will effect the calculation later on. Divide statements with commas.

```
x = `1, y = `2
[Σ] Done
x ++, y ++
[Σ] Done
x
[Σ] 2
y
[Σ] 2 
```

**iterators:**

To iterate a calculation, enclose the loop in [brackets]. As of right now, this just iterates six times (a bit goofy I reckon). Once conditional jumps are implemented, I'll add a `break` command. Your loops should probably contain non-returning statements...

```
x = 0, n = 1
[Σ] Done
[n = n + 1, x = x + n]
[Σ] Done
n
[Σ] 7
x
[Σ] 21
```