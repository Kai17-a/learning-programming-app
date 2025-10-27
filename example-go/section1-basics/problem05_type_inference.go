// Problem: Type Inference with Short Declaration
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
// TODO: Use short variable declaration and let Go infer the types
    name := "Go Programming"
    version := 1.21
    isStable := true
    userCount := 1000000

// TODO: Print the values and their types using %T format verb
    fmt.Printf("name: %s (type: %T)\n", name, name)
    fmt.Printf("version: %f (type: %T)\n", version, version)
    fmt.Printf("isStable: %t (type: %T)\n", isStable, isStable)
    fmt.Printf("userCount: %d (type: %T)\n", userCount, userCount)

// TODO: Declare multiple variables with different inferred types
    x, y, z := 10, 3.14, "hello"

    fmt.Printf("x: %d (%T), y: %f (%T), z: %s (%T)\n", x, x, y, y, z, z)

// TODO: Try reassigning with different types (this should cause an error)
// Uncomment the line below to see the error
// name = 123  // This will cause a compile error
}
