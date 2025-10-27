// Problem: Variadic Functions
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

// TODO: Define a variadic function that sums integers
func sum(numbers ...int) int {

}

// TODO: Define a variadic function with mixed parameters
func logMessage(level string, messages ...string) {

}

// TODO: Define a function that takes a slice and uses variadic syntax
func average(numbers ...float64) float64 {

}

func main() {
// TODO: Call variadic function with multiple arguments
    total := sum(1, 2, 3, 4, 5)
    fmt.Printf("Sum: %d\n", total)

// TODO: Call variadic function with slice using ... operator
    nums := []int{10, 20, 30}
    total2 := sum(nums...)
    fmt.Printf("Sum of slice: %d\n", total2)

// TODO: Call variadic function with mixed parameters
    logMessage("INFO", "System started", "Database connected", "Ready to serve")

// TODO: Call with no variadic arguments
    logMessage("ERROR")

// TODO: Calculate average
    avg := average(85.5, 92.0, 78.5, 96.0)
    fmt.Printf("Average: %.2f\n", avg)
}
