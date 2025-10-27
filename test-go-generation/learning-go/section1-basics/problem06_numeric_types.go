// Problem: Numeric Types and Operations
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
// TODO: Declare variables of different integer sizes
    var int8Val int8 = 127      // Max value for int8
    var int16Val int16 = 32767  // Max value for int16
    var int32Val int32 = 2147483647
    var int64Val int64 = 9223372036854775807

// TODO: Declare unsigned integer variables
    var uint8Val uint8 = 255
    var uint16Val uint16 = 65535

// TODO: Declare floating point variables
    var float32Val float32 = 3.14159
    var float64Val float64 = 2.718281828459045

    fmt.Printf("Signed integers:\n")
    fmt.Printf("int8: %d, int16: %d, int32: %d, int64: %d\n",
    int8Val, int16Val, int32Val, int64Val)

    fmt.Printf("Unsigned integers:\n")
    fmt.Printf("uint8: %d, uint16: %d\n", uint8Val, uint16Val)

    fmt.Printf("Floating point:\n")
    fmt.Printf("float32: %f, float64: %f\n", float32Val, float64Val)

// TODO: Perform arithmetic operations
    sum := int32Val + int32(int16Val)  // Type conversion needed
    product := float64Val * float64(float32Val)

    fmt.Printf("Sum: %d, Product: %f\n", sum, product)
}
