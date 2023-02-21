h## rs-pairs

rs-pairs works like pairs in other programming language. Ofcourse, rust tuple type can be used
to achieve ```some``` the basic function of this crate. Using the index of the tuple type, 
which is not natural as proposed using rust like so:

```
let tuple: (&str, i32) = ("rust-lang", 12_i32);
```

To get the "elements" in this variable, you might have to do:
``` 
assert_eq!(tuple.0, "rust-lang");
assert_eq!(tuple.1, 12);
```

*OR*

```
let (lang, age) = tuple;
println!("My favorite language is: {}, and it's {} years old", lang, age);
```

#### Using rs-pairs
```use rs-pairs::Pairs;

   let tuple: Pairs<&str, i32> = Pairs::new("rust-lang", 12);
   assert_eq!(tuple.first(), "rust-lang");
   assert_eq!(tuple.second(), 12);
```

Using methods, like `first`, and `second`, on an object of Pairs feels more natural, than 
calling on "index" function-like on tuple as provided by rust. 

