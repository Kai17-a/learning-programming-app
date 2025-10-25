package main

import "fmt"

// Simple function
func greet(name string) string {
    return fmt.Sprintf("Hello, %s!", name)
}

// Function with multiple parameters and return values
func calculate(a, b int) (int, int) {
    sum := a + b
    product := a * b
    return sum, product
}

// Function with named return values
func divide(a, b float64) (result float64, err error) {
    if b == 0 {
        err = fmt.Errorf("division by zero")
        return
    }
    result = a / b
    return
}

func main() {
    // Call simple function
    message := greet("Go")
    fmt.Println(message)
    
    // Call function with multiple returns
    sum, product := calculate(5, 3)
    fmt.Printf("Sum: %d, Product: %d\n", sum, product)
    
    // Call function with error handling
    result, err := divide(10, 2)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
    } else {
        fmt.Printf("Result: %.2f\n", result)
    }
    
    // Test division by zero
    _, err = divide(10, 0)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
    }
}