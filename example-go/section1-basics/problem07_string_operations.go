// Problem: String Operations and Literals
// Topic: Data Types
// Difficulty: 2

package main

import "fmt"

func main() {
// TODO: Declare strings using different literal types
    regularString := "Hello, World!"
    rawString := `This is a raw string
    with multiple lines
    and \n escape sequences are not interpreted`

// TODO: String concatenation
    firstName := "John"
    lastName := "Doe"
    fullName := firstName + " " + lastName

// TODO: String length and indexing
    message := "Go Programming"
    length := len(message)
    firstChar := message[0]  // Get first character (byte)

    fmt.Printf("Regular string: %s\n", regularString)
    fmt.Printf("Raw string: %s\n", rawString)
    fmt.Printf("Full name: %s\n", fullName)
    fmt.Printf("Message: %s (length: %d)\n", message, length)
    fmt.Printf("First character: %c (byte value: %d)\n", firstChar, firstChar)

// TODO: String formatting with different verbs
    age := 25
    height := 5.9
    fmt.Printf("Name: %s, Age: %d, Height: %.1f feet\n", fullName, age, height)

// TODO: Unicode string
    unicodeString := "Hello, 世界! 🌍"
    fmt.Printf("Unicode string: %s (byte length: %d)\n", unicodeString, len(unicodeString))
}
