<img src="./sigma.png">

<br/><br/>

<img src="./Solvation.png">

<br/><br/>

I will make a fully capable graphing calculator in Rust and you can't stop me.

As always, I'm not using libraries or any of that. Everything here is homemade :)

# **how to use**

If you have Rust installed and you can build from the source code, good for you.

If you don't just take the *L* for now and I'll have a binary release at some point.

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

To declare a new variable, use the `var` keyword, followed by its name and value. This value can be an expression, and `complete()` will simplify it. Variable and function names are stored not as a `String`, but instead as a `[char; 5]`. This means that while you can still use >5-character names, the code will only store the first 5 characters, meaning it will not be able to distinguish between names `longname2` and `longname1`, and treat them as the same variable or function.

```
var x 4
var λ 3 + 2
x * λ
[Σ] 20
```

Slight adjustments had to be made, but names work with Unicode characters. After a variable has been declared, it can be re-assigned with the `=` operator. Note that nothing in this program can return nothing (yet), and so re-assignment expressions return the value that is assigned.

```
var re 3
re = re + 1
[Σ] 4
re
[Σ] 4
```

**functions:**

To declare a function, use keyword `def`, followed by `name(inp1, inp2, ...)`, then the contents of the function. Solvation will analyze the input names and replace any instances of them with an input token. The commas here are important as they are used to separate inputs instead of spaces.

This example computes the magnitude of a diagonal line set by two inputs.

```
def mag(x, y) (x * x + y * y) ^ 0.5
mag(1,2)
[Σ] 2.2360198347089875
mag(1,1)
1.4142134378915125
```