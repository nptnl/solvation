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

**iterators:**

To iterate a calculation, enclose the loop in [brackets]. As of right now, this just iterates six times. Once conditional jumps are implemented, I'll add a `break` command. This should probably include a re-assignment...

```
x = 0
[Σ] Done
n = 1
[Σ] Done
[n = n + 1, x = x + n]
[Σ] Done
n
[Σ] 7
x
[Σ] 21
```