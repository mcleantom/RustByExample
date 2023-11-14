you can create a library in rust by calling the command:

```
rustc --crate-type=lib rary.rs
```

you can link a create to this library using --extern flag

```
rustc executable.rs --extern rary=library.rlib && ./executable
```