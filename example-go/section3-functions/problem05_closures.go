// Problem: Closures and Variable Capture
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

// TODO: Create a function that returns a closure
func makeCounter() func() int {

}

// TODO: Create a closure that captures multiple variables
func makeCalculator(initial int) (func(int) int, func() int) {

}

func main() {
// TODO: Use the counter closure
    counter1 := makeCounter()
    counter2 := makeCounter()

    fmt.Printf("Counter1: %d\n", counter1())
    fmt.Printf("Counter1: %d\n", counter1())
    fmt.Printf("Counter2: %d\n", counter2())
    fmt.Printf("Counter1: %d\n", counter1())

// TODO: Use the calculator closures
    add, getValue := makeCalculator(10)

    fmt.Printf("Initial value: %d\n", getValue())
    add(5)
    fmt.Printf("After adding 5: %d\n", getValue())
    add(-3)
    fmt.Printf("After adding -3: %d\n", getValue())

// TODO: Demonstrate closure capturing loop variables
    functions := make([]func() int, 3)

// Incorrect way (all closures will capture the same variable)
    for i := 0; i < 3; i++ {
        functions[i] = func() int {
            return i  // This captures the loop variable
        }
    }

    fmt.Println("Incorrect closure capture:")
    for j, fn := range functions {
        fmt.Printf("Function %d returns: %d\n", j, fn())
    }

// TODO: Correct way to capture loop variables
    correctFunctions := make([]func() int, 3)
    for i := 0; i < 3; i++ {
        i := i  // Create a new variable in each iteration
        correctFunctions[i] = func() int {
            return i
        }
    }

    fmt.Println("Correct closure capture:")
    for j, fn := range correctFunctions {
        fmt.Printf("Function %d returns: %d\n", j, fn())
    }
}
