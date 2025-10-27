// Problem: Understanding Zero Values
// Topic: Variables
// Difficulty: 1

package main

import "fmt"

func main() {
// TODO: Declare variables without initialization to see their zero values
    var intZero int
    var floatZero float64
    var stringZero string
    var boolZero bool

// TODO: Declare a pointer and see its zero value
    var pointerZero *int

    fmt.Printf("Zero values:\n")
    fmt.Printf("int: %d\n", intZero)
    fmt.Printf("float64: %f\n", floatZero)
    fmt.Printf("string: '%s'\n", stringZero)
    fmt.Printf("bool: %t\n", boolZero)
    fmt.Printf("pointer: %v\n", pointerZero)

// TODO: Check if string is empty and pointer is nil
    if stringZero == "" {
        fmt.Println("String zero value is empty string")
    }

    if pointerZero == nil {
        fmt.Println("Pointer zero value is nil")
    }
}
