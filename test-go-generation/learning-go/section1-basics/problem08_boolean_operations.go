// Problem: Boolean Type and Logical Operations
// Topic: Data Types
// Difficulty: 1

package main

import "fmt"

func main() {
// TODO: Declare boolean variables
    var isStudent bool = true
    var hasLicense bool = false

// TODO: Boolean operations
    canDrive := hasLicense && !isStudent  // Logical AND and NOT
    needsPermission := isStudent || !hasLicense  // Logical OR

// TODO: Comparison operations that result in boolean
    age := 20
    isAdult := age >= 18
    isTeenager := age >= 13 && age <= 19

    fmt.Printf("Is student: %t\n", isStudent)
    fmt.Printf("Has license: %t\n", hasLicense)
    fmt.Printf("Can drive: %t\n", canDrive)
    fmt.Printf("Needs permission: %t\n", needsPermission)
    fmt.Printf("Is adult: %t\n", isAdult)
    fmt.Printf("Is teenager: %t\n", isTeenager)

// TODO: Boolean in conditional context
    if isAdult {
        fmt.Println("Person is an adult")
    }

    if !hasLicense {
        fmt.Println("Person needs to get a license")
    }
}
