// Problem: Variable Scope and Shadowing
// Topic: Variables
// Difficulty: 2

package main

import "fmt"

// TODO: Package-level variable
var globalVar string = "I'm global"

func main() {
// TODO: Function-level variable
    var localVar string = "I'm local to main"

    fmt.Printf("Global variable: %s\n", globalVar)
    fmt.Printf("Local variable: %s\n", localVar)

// TODO: Block scope
    {
        var blockVar string = "I'm in a block"
// TODO: Shadow the local variable
        localVar := "I'm shadowing the outer localVar"

        fmt.Printf("Inside block - localVar: %s\n", localVar)
        fmt.Printf("Inside block - blockVar: %s\n", blockVar)
    }

// blockVar is not accessible here
    fmt.Printf("Outside block - localVar: %s\n", localVar)

// TODO: Loop scope
    for i := 0; i < 3; i++ {
// i is only accessible within this loop
        fmt.Printf("Loop iteration: %d\n", i)
    }

// i is not accessible here

// TODO: If statement scope
    if x := 10; x > 5 {
        fmt.Printf("x in if statement: %d\n", x)
    }

// x is not accessible here
}
