package main

import "fmt"

func main() {
    // Variable declarations
    var name string = "Go Programming"
    age := 30
    isActive := true
    
    // Print variables
    fmt.Printf("Name: %s\n", name)
    fmt.Printf("Age: %d\n", age)
    fmt.Printf("Active: %t\n", isActive)
    
    // Constants
    const pi = 3.14159
    fmt.Printf("Pi: %.2f\n", pi)
}