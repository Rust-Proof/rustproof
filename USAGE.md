# To Use Rust-Proof
Users should place the "condition" attribute tag above any function they should wish to test, along with a precondition and postcondition. If a user has a function "foo()" they can specify that Rust-Proof should analyze it like so:

```
#condition[pre="P",post="Q"]
foo() { ... }
```

...where "P" is a string containing the precondition, and "Q" is a string containing the postcondition. Neither string may be empty. If a user does not wish to specify either, they can enter the string "true".

# How to format preconditions and postconditions
Pre- and postconditions are made of boolean logical expressions. These expressions are composed of operands and operators. When all is said and done, the expressions should resolve to a boolean value.

## Operands
Currently Rustproof will accept boolean literals (true, false), rust integer types (u8, i64, isize, etc.), and variables of either of those types. Variables are named just like Rust identifiers. Expressions can also be operands, if they resolve to the correct type. 
You must identify the type of your object with Rust-like syntax (except for "true" or "false"). Casting is not supported.
In the precondition, the user can only reference variables that are arguments to the function in question. In the postcondition, only the returned value can be referenced, as a special variable called "return".

## Operators
There are three ways to think about operators: how many operands they work on, what types of operands they can work with, and what type an expression involving them resolves to. Currently there is no operator precedence, so complex expressions must be wrapped in parentheses to determine associativity. Rustproof uses infix notation.

| Operator | Name                        | Number of operands | Operand type  | Resolution type |
|----------|-----------------------------|--------------------|---------------|-----------------|
| +        | Addition                    | 2                  | Integer       | Integer         |
| -        | Subtraction                 | 2                  | Integer       | Integer         |
| *        | Multiplication              | 2                  | Integer       | Integer         |
| /        | Division                    | 2                  | Integer       | Integer         |
| %        | Modulus / Remainder         | 2                  | Integer       | Integer         |
| \|       | Bitwise Or                  | 2                  | Any Primitive | Any Primitive   |
| &        | Bitwise And                 | 2                  | Any Primitive | Any Primitive   |
| ^        | Bitwise Exclusive Or / XOR  | 2                  | Any Primitive | Any Primitive   |
| <<       | Bitwise Left Shift          | 2                  | Any Primitive | Any Primitive   |
| >>       | Bitwise Right Shift         | 2                  | Any Primitive | Any Primitive   |
| <        | Less Than                   | 2                  | Integer       | Boolean         |
| <=       | Less Than Or Equal          | 2                  | Integer       | Boolean         |
| >        | Greater Than                | 2                  | Integer       | Boolean         |
| >=       | Greater Than Or Equal       | 2                  | Integer       | Boolean         |
| ==       | Equality                    | 2                  | Integer       | Boolean         |
| !=       | Inequality                  | 2                  | Integer       | Boolean         |
| &&       | Logical And                 | 2                  | Boolean       | Boolean         |
| \|\|     | Logical Or                  | 2                  | Boolean       | Boolean         |
| AND      | Conjunction                 | 2                  | Boolean       | Boolean         |
| OR       | Disjunction                 | 2                  | Boolean       | Boolean         |
| XOR      | Exclusive Disjunction / XOR | 2                  | Boolean       | Boolean         |
| IMPLIES  | Implication                 | 2                  | Boolean       | Boolean         |
| EQUIV    | Equivalence                 | 2                  | Boolean       | Boolean         |
| -        | Negation                    | 1                  | Integer       | Integer         |
| !        | Bitwise Not                 | 1                  | Any Primitive | Any Primitive   |
| NOT      | Logical Negation            | 1                  | Boolean       | Boolean         |

__Note__: The "&&", "||", and "!" operators are treated identically to the "AND", "OR", and "NOT" operators, respectively. "AND" and "OR" are added as conventions to make clear what is and is not meant to be a Rust-like expression, and "!" is overriden in Rust to be both logical and bitwise negation, since bitwise negation on a boolean primitive type amounts to the same thing.

__Operator precedence is as follows__ (more tightly binding first):
- (Unary), !, NOT
*, /, %
+, - (Binary)
^, &, |, <<, >>
&&, ||
AND, OR, XOR, IMPLICATION, BIIMPLICATION

__Examples__:

```
x: i32 > 5: i32
! true
(y: u64 <= b: u64) OR ((b: u64 + 4: u64) > 8: u64)
return: bool AND (!false)
```