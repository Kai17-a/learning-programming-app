// Problem: Type Conversion and Casting
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
// TODO: Numeric type conversions
    var intVal int = 42
    var floatVal float64 = 3.14159

// Convert int to float
    intToFloat := float64(intVal)

// Convert float to int (truncates decimal part)
    floatToInt := int(floatVal)

    fmt.Printf("Original int: %d, converted to float: %f\n", intVal, intToFloat)
    fmt.Printf("Original float: %f, converted to int: %d\n", floatVal, floatToInt)

// TODO: String conversions
    var char byte = 65  // ASCII value for 'A'
    charToString := string(char)

    fmt.Printf("Byte %d as string: %s\n", char, charToString)

// TODO: Demonstrate precision loss
    var bigFloat float64 = 123.456789
    var smallFloat float32 = float32(bigFloat)

    fmt.Printf("Original float64: %f\n", bigFloat)
    fmt.Printf("Converted to float32: %f\n", smallFloat)

// TODO: Integer overflow demonstration
    var maxInt8 int8 = 127
// Uncomment to see overflow behavior
// var overflow int8 = maxInt8 + 1

    fmt.Printf("Max int8: %d\n", maxInt8)
    fmt.Printf("Adding 1 would cause overflow\n")
}
