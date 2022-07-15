# Inputs

rusty-cli provides a ton of different inputs. It uses the 
inputs from <a href="https://docs.rs/dialoguer/0.10.1/dialoguer/">dialoguer</a>
as base and creates some wrapped inputs that are much easier to
use and maintain.

## Confirm Input 

```rust
let yes_no = ConfirmInput::get_value("Yes or no?".to_string());
println!("{}", yes_no);
```

The `get_value` method defined on the `ConfirmInput` struct takes one single 
parameter that defines the prompt that is shown if the input is opened.

## MultiSelect Input

```rust
let items = vec!["Option 1".to_string(), "Option 2".to_string()];

    let indexes = MultiSelectInput::get_selected_indexes(items.clone());
    println!("{:?}", indexes);

    let values = MultiSelectInput::get_selected_values(items);
    println!("{:?}", values);
```

The `get_selected_indexes` method defined on the `MultiSelectInput` 
struct takes only the items (`Vec<String>`) as parameter and returns the index values of the 
vector. The `get_selected_values` takes the same parameter, but returns the absolute 
values from the vector.

## Password Input

```rust
let pwd = PasswordInput::get_value(Some("Password".to_string()), false);
println!("{}", pwd.unwrap());
```
The method `get_value` takes two parameters. The first one is an `Option<String>` and defines 
the prompt that should be shown. If it is `None` no prompt is shown.
The second parameter indicates whether the password should be retyped before submitting.

## Select Input

```rust
let items = vec!["Option 1".to_string(), "Option 2".to_string()];

let index = SelectInput::get_index(items.clone());
println!("{}", index.unwrap());

let value = SelectInput::get_value(items);
println!("{}", value);
```

The method `get_index` takes the items as parameter and returns 
the index in the vector of the items. The method `get_value` takes the 
same parameters, but returns the direct value from the items.

## Text Input

```rust
let value = TextInput::get_value(Some("Frage".to_string()));
println!("{}", value);
```

The method `get_value` takes an `Option<String>` as parameter. It defines the 
prompt that should be shown. If it is `None` no prompt is shown.

## Vim Input 

```rust
let input_value = VimInput::get_value("Edit me".to_string());
println!("{}", input_value.unwrap());
```

The method `get_value` takes a single `String` as parameter. The string 
is edited in the vim editor that is opened by the cli and the 
result is returned by the method.