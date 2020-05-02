# Ijssel

Tiny LLVM-based compiler.

```
fn main do
    add(100, 1000)
end

# Adds a to b
fn add(a, b) do
    a + b
end
```

```
USAGE:
    ijssel [FLAGS] [OPTIONS] <FILE>

FLAGS:
        --debug      Whether to perform optimisations
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --type <file-type>    Output format. [default: object]  [possible values: object, asm]
    -o, --output <output>     Output to write object or assembly to. If omitted this will default to the input file name
                              with an extension corresponding the `type`

ARGS:
    <FILE>    An ijssel source file
```
