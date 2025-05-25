# Lox Language Feature Roadmap

This document outlines potential features that could be added to the Lox language to enhance its capabilities and usability. It draws inspiration from the "Crafting Interpreters" book by Robert Nystrom and common features in other modern programming languages.

## 1. Language Core Enhancements
### 1.1. Additional Operators
- **Modulo Operator:** Add `%%` for remainder after division.
- **Conditional (Ternary) Operator:** Implement `condition ? exprIfTrue : exprIfFalse` for concise conditional expressions.
- **Bitwise Operators:** Introduce `&` (AND), `|` (OR), `^` (XOR), `~` (NOT) for low-level bit manipulation.
- **Shift Operators:** Add `<<` (left shift) and `>>` (right shift).

### 1.2. Constants
- Introduce a `const` keyword to declare variables whose values cannot be reassigned after initialization.
  ```lox
  const PI = 3.14159;
  // PI = 3; // This would be an error
  ```

### 1.3. Comments
- **Block Comments:** Add support for C-style block comments `/* ... */` in addition to existing `//` line comments.

### 1.4. Numerical Types and Literals
- **Distinct Integer Type:** Consider adding a dedicated integer type, which could offer performance benefits or specific behaviors (e.g., overflow handling) separate from floating-point numbers.
- **Expanded Number Literals:** Support for hexadecimal (`0x...`), binary (`0b...`), and octal (`0o...`) number literals.
## 2. Control Flow
### 2.1. `do-while` Loops
- Implement `do { ... } while (condition);` loops that execute the body at least once.
  ```lox
  var i = 0;
  do {
    print i;
    i = i + 1;
  } while (i < 0); // Prints 0, then condition is false
  ```

### 2.2. Enhanced `for` Loops
- **`for-in` / `foreach` Loops:** Introduce a way to iterate over collections (e.g., future built-in lists or keys of maps).
  ```lox
  // Assuming 'myList' is a list [1, 2, 3]
  // for (item in myList) {
  //   print item;
  // }
  ```
  (Syntax to be determined, depends on collection types)

### 2.3. Loop Control Statements
- **`break` Statement:** Add `break;` to exit the innermost enclosing loop immediately.
- **`continue` Statement:** Add `continue;` to skip the current iteration and proceed to the next one in the innermost enclosing loop.

### 2.4. `switch` / `case` Statements
- Implement `switch` statements for multi-way branching based on the value of an expression.
  ```lox
  // switch (value) {
  //   case 1:
  //     print "One";
  //     break;
  //   case 2:
  //     print "Two";
  //     break;
  //   default:
  //     print "Other";
  // }
  ```
  (Syntax and fall-through behavior to be determined)
## 3. Data Structures
### 3.1. Built-in List/Array Type
- Introduce a native list or array data structure for ordered collections of items.
- Should support dynamic sizing.
- Operations might include:
  - Access by index (`myList[i]`)
  - Appending elements (`myList.add(item)` or `myList.push(item)`)
  - Removing elements
  - Getting length (`myList.length()` or `len(myList)`)
  - Slicing
  ```lox
  // var numbers = [1, 2, 3, 4];
  // print numbers[1]; // 2
  // numbers.add(5);
  // print numbers.length(); // 5
  ```
  (Syntax for literals and methods to be determined)

### 3.2. Built-in Map/Dictionary/Hash Type
- Introduce a native map, dictionary, or hash table data structure for key-value pairs.
- Keys could be strings or other hashable types.
- Operations might include:
  - Access by key (`myMap["key"]` or `myMap.get("key")`)
  - Setting values (`myMap["key"] = value` or `myMap.set("key", value)`)
  - Removing entries
  - Checking for key existence
  - Getting size
  ```lox
  // var person = {"name": "Loxley", "age": 5};
  // print person["name"]; // "Loxley"
  // person["age"] = 6;
  // print person.exists("occupation"); // false
  ```
  (Syntax for literals and methods to be determined)
## 4. Object-Oriented Programming (OOP)
### 4.1. Static Methods and Properties
- Allow classes to have static methods and properties that are called on the class itself, not on instances.
  ```lox
  // class Math {
  //   static PI = 3.14159;
  //   static abs(n) {
  //     if (n < 0) return -n;
  //     return n;
  //   }
  // }
  // print Math.PI;
  // print Math.abs(-5);
  ```
  (Syntax for defining static members to be determined)

### 4.2. Getters and Setters
- Introduce special syntax for getter and setter methods, allowing computed properties or controlled access to fields.
  ```lox
  // class Circle {
  //   init(radius) {
  //     this._radius = radius;
  //   }
  //
  //   get radius() {
  //     return this._radius;
  //   }
  //
  //   set radius(value) {
  //     if (value < 0) throw "Radius cannot be negative.";
  //     this._radius = value;
  //   }
  //
  //   get area() {
  //     return 3.14159 * this._radius * this._radius;
  //   }
  // }
  //
  // var c = Circle(5);
  // print c.radius; // 5
  // print c.area;   // Calls getter
  // c.radius = 10;  // Calls setter
  ```
  (Syntax for `get` and `set` keywords to be determined)

### 4.3. "Pure" Object-Oriented Primitives
- Explore making primitive types (numbers, booleans, strings, nil) behave as true objects with methods.
  This would unify the type system, as noted in "Crafting Interpreters".
  ```lox
  // print 5.toString();
  // print "hello".length();
  ```
  (This is a significant change and would require careful consideration)

### 4.4. Class Immutability / `final`
- Consider ways to declare classes as `final` (cannot be subclassed).
- Consider ways to declare methods as `final` (cannot be overridden).

### 4.5. Interfaces or Mixins
- Explore mechanisms for defining contracts (interfaces) or reusing code across class hierarchies (mixins) to provide more flexible code sharing than single inheritance alone.
## 5. Functions
### 5.1. Arrow Functions / Lambda Syntax
- Introduce a concise syntax for anonymous functions, often called arrow functions or lambdas.
  ```lox
  // var add = (a, b) => a + b;
  // print add(3, 4); // 7
  //
  // // Useful for callbacks
  // var numbers = [1, 2, 3];
  // var doubled = numbers.map(n => n * 2); // Assuming lists and a map method
  ```
  (Syntax to be determined)

### 5.2. Default Parameter Values
- Allow function parameters to have default values if an argument is not provided.
  ```lox
  // fun greet(name, greeting = "Hello") {
  //   print greeting + ", " + name + "!";
  // }
  //
  // greet("Reader");           // "Hello, Reader!"
  // greet("Friend", "Hi");     // "Hi, Friend!"
  ```

### 5.3. Rest Parameters / Varargs
- Allow functions to accept a variable number of arguments as a list.
  ```lox
  // fun sumAll(...numbers) {
  //   var total = 0;
  //   for (num in numbers) { // Assuming for-in and list-like 'numbers'
  //     total = total + num;
  //   }
  //   return total;
  // }
  // print sumAll(1, 2, 3, 4); // 10
  ```
  (Syntax for `...` and type of the collected arguments to be determined)

### 5.4. Named Arguments
- Consider allowing arguments to be passed by parameter name, which can improve clarity for functions with many parameters.
  ```lox
  // fun createWindow(width, height, title = "Untitled", visible = true) { /* ... */ }
  // createWindow(width: 800, height: 600, title: "My App");
  ```
  (Syntax to be determined)
## 6. Error Handling
### 6.1. `try-catch-finally` Blocks
- Implement structured exception handling with `try`, `catch`, and `finally` blocks.
- Allow user code to throw and catch custom errors/exceptions.
  ```lox
  // fun readFile(path) {
  //   if (!fileExists(path)) { // Assuming a fileExists function
  //     throw "File not found: " + path;
  //   }
  //   // ... proceed to read file ...
  // }
  //
  // try {
  //   var content = readFile("mydata.txt");
  //   print "File content: " + content;
  // } catch (e) {
  //   print "Error: " + e;
  // } finally {
  //   print "Cleanup actions here.";
  // }
  ```
  (Details of exception objects and `throw` statement to be determined)

### 6.2. Custom Error Objects
- Allow defining custom error types, possibly inheriting from a base `Error` class, for more specific error handling.
  ```lox
  // class NetworkError < Error {
  //   init(message, address) {
  //     super.init(message); // Assuming Error base class and init
  //     this.address = address;
  //   }
  // }
  //
  // // throw NetworkError("Connection timed out", "example.com");
  ```
## 7. Modules and Namespacing
### 7.1. Module System
- Develop a module system to allow code to be organized into separate files and reused.
- This would involve:
  - Defining how to import symbols (functions, classes, variables) from other files.
  - Defining how to export symbols from a module.
  - Handling circular dependencies.
  ```lox
  // --- lib/math.lox ---
  // export fun add(a, b) { return a + b; }
  // export const PI = 3.14159;

  // --- main.lox ---
  // import math;
  // print math.add(1, 2); // 3
  // print math.PI;
  //
  // // Or, with named imports:
  // import {add, PI} from math;
  // print add(3, 4); // 7
  // print PI;
  ```
  (Syntax for `import` and `export` to be determined)

### 7.2. Namespaces
- While modules would provide file-level namespacing, consider if a more general `namespace` keyword or construct is needed for grouping code within a single file or for more complex organizational structures. This might be less critical if modules are powerful enough.
## 8. Standard Library
## 9. Miscellaneous
