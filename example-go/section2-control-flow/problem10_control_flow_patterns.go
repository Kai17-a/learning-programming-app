// Problem: Common Control Flow Patterns
// Topic: Control Flow
// Difficulty: 3

package main

import "fmt"

func main() {
// TODO: Implement a simple menu system using switch
    choice := 2

// TODO: Implement input validation using for loop and if
    attempts := 0
    maxAttempts := 3
    validInput := false

// TODO: Implement a simple state machine using switch in loop
    state := "start"
    steps := 0
    maxSteps := 10

    fmt.Printf("Menu choice: %d\n", choice)
    fmt.Printf("Validation attempts: %d/%d, Valid: %t\n", attempts, maxAttempts, validInput)
    fmt.Printf("State machine: %s, Steps: %d/%d\n", state, steps, maxSteps)
}
