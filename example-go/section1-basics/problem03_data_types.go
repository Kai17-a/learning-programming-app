// Problem: Basic Data Types
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
// TODO: Declare variables of different numeric types
    var smallInt int8
    var bigInt int64
    var floatNum float32
    var doubleNum float64

// TODO: Assign appropriate values to each variable

// TODO: Declare string and boolean variables
    var message string
    var isActive bool

// TODO: Assign values to string and boolean

// TODO: Perform type conversion between int and float
    var intValue int = 42
    var floatValue float64
// Convert intValue to float64 and assign to floatValue

    fmt.Printf("Int8: %d, Int64: %d\n", smallInt, bigInt)
    fmt.Printf("Float32: %f, Float64: %f\n", floatNum, doubleNum)
    fmt.Printf("String: %s, Boolean: %t\n", message, isActive)
    fmt.Printf("Converted: %f\n", floatValue)
}
