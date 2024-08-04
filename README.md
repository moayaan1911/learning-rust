# Learning Rust

An overview of my journey into Rust programming, covering fundamental concepts and syntax.

#### Topics I learnt

**Day 1:**

- **Variables and Data Types**

  - Explored variable declaration and initialization.
  - Example: `let num: u8 = 10;` declares an unsigned 8-bit integer.

- **Printing to Console**

  - Learned how to output text and variables to the console using the `println!` macro.
  - Example: `println!("Hello, China! {}", num);` prints "Hello, China! 10".

- **Strings**

  - **String Literals (`&str`):**
    - Immutable, stored in stack memory, and static.
    - Example: `let string_literal_static: &str = "Hello Everyone";`
    - Output: `println!("{}", string_literal_static);` prints "Hello Everyone".
  - **String Type (`String`):**
    - Mutable, stored in heap memory, and dynamic.
    - Example: `let mut string_literal_dynamic: String = String::from("Hello Everyone");`
    - Can be modified using methods like `push_str`.
    - Output:
      ```rust
      string_literal_dynamic.push_str(", China!");
      println!("{}", string_literal_dynamic);  // prints "Hello Everyone, China!"
      ```

- **Tuples**

  - Learned to group different types into a single compound type.
  - Example: `let emp_value: (&str, u8) = ("Tom", 32);`
  - Accessed tuple elements using dot notation.
  - Output: `println!("Employee name is {} and age is {}", emp_value.0, emp_value.1);` prints "Employee name is Tom and age is 32".

- **Functions**
  - Defined and invoked functions.
  - Example:
    ```rust
    fn sum(number1: u8, number2: u8) -> u8 {
        return number1 + number2;
    }
    let number1: u8 = 10;
    let number2: u8 = 20;
    let result: u8 = sum(number1, number2);
    println!("Sum of {} and {} is {}", number1, number2, result);  // prints "Sum of 10 and 20 is 30"
    ```

**Day 2:**

- **Ownership**

  - Understanding how Rust handles memory through ownership rules.
  - Example: `let s1: String = get_string();`
  - The `get_string` function returns a `String` which is then owned by `s1`.

- **Move Semantics**

  - Example:
    ```rust
    let s2: String = String::from("World");
    let s3: String = send_get_string(s2);
    // println!("The value of s2 is {}", s2); // THIS WILL THROW ERROR
    println!("The value of s3 is {}", s3);
    ```
  - `s2` is moved to `send_get_string`, so `s2` is no longer valid.
  - Attempting to use `s2` after it has been moved will cause a compile-time error.

- **Function Return and Ownership**

  - Example:
    ```rust
    fn get_string() -> String {
        let s: String = String::from("Hello");
        return s;
    }
    ```
  - The `get_string` function returns ownership of the `String`.

- **Ownership Transfer Between Functions**
  - Example:
    ```rust
    fn send_get_string(s: String) -> String {
        return s;
    }
    ```
  - Ownership of `s` is transferred into the function and then returned, transferring ownership back to the caller.
