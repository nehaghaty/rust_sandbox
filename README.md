### rust_sandbox
This is where my attempt to learn rust will be documented

Shadowing
- declare a variable with the same name as the previous variable
- 1st variable is shadowed by the 2nd
- different from mut. We get compiler error if let is not used for the shadow
- since a new variable is being created, we can chage the type of the shadow variable, Example:

```
let spaces = "  ";
let spaces = spaces.lem();  
```
-  cannot do the above with `mut`
---
Expression vs Statement
- Expression: has no semicolon, has a return value
- Statement: has a semicolon, doesn't return a value
---
Ownership
- memory safety without gc
- String:

```
let s= "hello"; //string literal
let s= String::from("hello"); //string => can be mutated
```
- if there is heap allocation involved, `Drop` trait is implemented. 
- If drop trait is implemented, `Copy` trait will not be implemented.
- If `Copy` trait is implemented the first variable will not become invalid in the below code.
- In the case of stack only data (integers), deep and shallow copy are the same and the below code is valid:

```
let x= 5;
let y=x;
println!("{} {}", x,y)
```
- In the case of String (which involves heap alloc), a shallow copy (i.e capacity, size, pointer to heap are copied) is made. Hence, when the value of a string is bound to a new variable, the first variable becomes invalid. So the below code not valid:

```
let s1 = String::from("Hello");
let s2=s1;
println!("{} {}", s1,s2);
```
- For data types involving heap allocation, deep copy is implemented using `clone`.

---

Borrowing
- prevent data races at compile time: only one mutable reference to a piece of data is allowed in a particular scope.
- passing references as function parameters is borrowing
- when the scope of a reference variable ends, it doesn't call drop since it never owned the value/
- references are immutable by default
- they can be made mutable using `&mut`
- scope of a reference ends after the last time it is used
- only one mutable reference OR amny immutatable references to a var are allowed in a scope
- `slice`: string slice is represented as `&str` (immutable reference)
- stirng literals are slices 


---

- `enumerate` : wraps result of `iter` and makes each element a tuple with index and reference to element

---

Structs
- associated functions: don't have an instance of the struct but are bound to the struct, allow namespacing functionality
- methods: like functions, have `self`
- rust doesn't have `->`. it has automatic referencing and dereferencing since it has a clear receiver: the type of self (`self`, `&self`, `&mut self`)

---
Enums
- 



