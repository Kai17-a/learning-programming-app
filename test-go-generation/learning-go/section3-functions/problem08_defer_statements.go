// Problem: Defer Statements
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Function demonstrating basic defer
func basicDefer() {
    fmt.Println("Function start")

// TODO: Add defer statements

    fmt.Println("Function middle")

// TODO: Add more defer statements

    fmt.Println("Function end")
}

// TODO: Function demonstrating defer with parameters
func deferWithParams() {
    x := 10

// TODO: Defer with current value of x

    x = 20

// TODO: Defer with updated value of x

    fmt.Printf("x at end of function: %d\n", x)
}

// TODO: Function demonstrating defer for cleanup
func fileOperation() {
    fmt.Println("Opening file...")

// TODO: Simulate file operations with defer cleanup

    fmt.Println("Processing file...")

// Simulate some work
    for i := 0; i < 3; i++ {
        fmt.Printf("Processing line %d\n", i+1)
    }

    fmt.Println("File processing complete")
}

// TODO: Function demonstrating defer in loops
func deferInLoop() {
    fmt.Println("Defer in loop example:")

// TODO: Show what happens with defer in a loop

}

func main() {
    fmt.Println("=== Basic Defer ===")
    basicDefer()

    fmt.Println("\n=== Defer with Parameters ===")
    deferWithParams()

    fmt.Println("\n=== Defer for Cleanup ===")
    fileOperation()

    fmt.Println("\n=== Defer in Loop ===")
    deferInLoop()

// TODO: Demonstrate defer order (LIFO - Last In, First Out)
    fmt.Println("\n=== Defer Order (LIFO) ===")

}
