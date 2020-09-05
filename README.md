# Scaffex 

A language-agnostic utility to **scaff**old **ex**ercise boilerplate files from
solution files. Handy for keeping instructor reference implementations and 
student-facing exercise files in sync.

## Installation
```
cargo install scaffex 
```

## Example

Given a path to a solution file or directory, `scaffex` will generate copies
of the solution files modulo any blocks of code delimited by a `START` and `END`
delimiter.

For example, given the following with `//#//Start` and `//#//End` as the `START` and
`END` delimiters respectively, the following
```rust
// example.rs
pub fn solution_implementation(s: String) -> bool {
    //#//Start
    s == "the answer"
    //#//End 
}
```
will generate:
```rust
// lib.rs
pub fn solution_implementation(s: String) -> bool {

}
```

## Usage

`scaffex` reads in config data from a `.scaffex.toml` file. Parameters that can
be configured include:

- `START` and `END` delimiters
- Path to the solution/example files
- Path where the generated boilerplate will go
- Optional comments/code snippets to replace removed code
- Optional files from the input directory that you want excluded from the generated 
output altogether

An example `.scaffex.toml` file might look like this:
```
# Replace removed code with the following
replace = "// Your code here"

# Exclude paths you don't want copied to the generated boilerplate
exclude = [
    ".meta/dont_include_me.md"
]

[parameters]
    [parameters.start]
    type = "string"
    message = "Specify the `START` delimiter"
    default = "//#//Start"
    
    [parameters.end]
    type = "string"
    message = "Specify the `END` delimiter"
    default = "//#//End"
    
    [parameters.src]
    type = "string"
    message = "Specify the location of the source file/directory"
    default = ".meta/example.rs"
    
    [parameters.dest]
    type = "string"
    message = "Specify the location of the destination file/directory"
    default = "src/lib.rs"
```

## Progress

- [ ] Generate a single exercise file from a single solution file.
- [ ] Generate an exercise directory from a solution directory.
