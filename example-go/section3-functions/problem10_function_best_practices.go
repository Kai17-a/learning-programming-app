// Problem: Function Best Practices
// Topic: Function Basics
// Difficulty: 2

package main

import (
"errors"
"fmt"
)

// TODO: Function with clear naming and single responsibility
func calculateCircleArea(radius float64) (float64, error) {

}

// TODO: Function with proper error handling
func divide(a, b float64) (float64, error) {

}

// TODO: Function with input validation
func createUser(name string, age int) (*Person, error) {

}

// TODO: Function with consistent return patterns
func findMaxValue(numbers []int) (int, bool) {

}

// TODO: Function with proper documentation
// calculateTax calculates the tax amount based on income and tax rate.
// It returns the tax amount and any error that occurred during calculation.
// Parameters:
//   - income: the gross income amount (must be >= 0)
//   - taxRate: the tax rate as a decimal (must be between 0 and 1)
// Returns:
//   - float64: the calculated tax amount
//   - error: any error that occurred during calculation
func calculateTax(income, taxRate float64) (float64, error) {

}

func main() {
// TODO: Test circle area calculation
    radius := 5.0
    area, err := calculateCircleArea(radius)
    if err != nil {
        fmt.Printf("Error calculating area: %v\n", err)
        } else {
            fmt.Printf("Circle area (radius %.1f): %.2f\n", radius, area)
        }

// TODO: Test division with error handling
        result, err := divide(10, 2)
        if err != nil {
            fmt.Printf("Division error: %v\n", err)
            } else {
                fmt.Printf("10 / 2 = %.2f\n", result)
            }

// Test division by zero
            _, err = divide(10, 0)
            if err != nil {
                fmt.Printf("Expected error: %v\n", err)
            }

// TODO: Test user creation with validation
            user, err := createUser("Alice", 25)
            if err != nil {
                fmt.Printf("User creation error: %v\n", err)
                } else {
                    fmt.Printf("Created user: %+v\n", user)
                }

// Test invalid user data
                _, err = createUser("", -5)
                if err != nil {
                    fmt.Printf("Expected validation error: %v\n", err)
                }

// TODO: Test max value finding
                numbers := []int{3, 7, 2, 9, 1}
                max, found := findMaxValue(numbers)
                if found {
                    fmt.Printf("Max value in %v: %d\n", numbers, max)
                    } else {
                        fmt.Println("No max value found (empty slice)")
                    }

// TODO: Test tax calculation
                    tax, err := calculateTax(50000, 0.25)
                    if err != nil {
                        fmt.Printf("Tax calculation error: %v\n", err)
                        } else {
                            fmt.Printf("Tax on $50,000 at 25%%: $%.2f\n", tax)
                        }
                    }
