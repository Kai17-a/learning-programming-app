// Problem: Function Types and Signatures
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Define a custom function type
type MathOperation func(int, int) int

// TODO: Define functions that match the MathOperation type
func add(a, b int) int {

}

func subtract(a, b int) int {

}

func multiply(a, b int) int {

}

// TODO: Define a function that takes a function as parameter
func calculate(a, b int, op MathOperation) int {

}

// TODO: Define a function that returns a function
func getOperation(opName string) MathOperation {

}

func main() {
    x, y := 10, 5

// TODO: Use functions directly
    fmt.Printf("%d + %d = %d\n", x, y, add(x, y))
    fmt.Printf("%d - %d = %d\n", x, y, subtract(x, y))

// TODO: Pass functions as arguments
    result1 := calculate(x, y, add)
    result2 := calculate(x, y, multiply)
    fmt.Printf("Calculate with add: %d\n", result1)
    fmt.Printf("Calculate with multiply: %d\n", result2)

// TODO: Get function from another function
    addOp := getOperation("add")
    subOp := getOperation("subtract")

    fmt.Printf("Dynamic add: %d\n", addOp(x, y))
    fmt.Printf("Dynamic subtract: %d\n", subOp(x, y))

// TODO: Create a slice of functions
    operations := []MathOperation{add, subtract, multiply}
    operationNames := []string{"add", "subtract", "multiply"}

    for i, op := range operations {
        result := op(x, y)
        fmt.Printf("%s: %d\n", operationNames[i], result)
    }
}
