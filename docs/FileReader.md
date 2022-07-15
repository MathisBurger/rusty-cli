# File Reader

rusty-cli provides a file reader that allows it to read and write files
in the OS app data dir.

## Creating a new file reader

```rust
FileReader::new("my-cli-app".to_string())
```

The provided parameter of the initializer method `FileReader::new()`
defines the name of the folder that should be used as data dir.

## Reading files

```rust
let reader = FileReader::new("my-cli-app".to_string());
let content = reader.read_file_to_string("filename.txt".to_string());
println!("{}", content);
```

The method `read_file_to_string` takes only the filename and returns the content of the
file. If the file does not exist, it is created by the file reader.

## Writing files

```rust
let reader = FileReader::new("my-cli-app".to_string());
reader.write_string_to_file("filename.txt".to_string(), "content".to_string());
```

The method `write_string_to_file` takes the filename and the content that should be written as
parameter. If the file does not exist, the file reader creates the file and writes the
content in it.