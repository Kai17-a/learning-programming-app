// Problem: Basic Function Definitions
// Topic: Function Basics
// Difficulty: 1

package main

import "fmt"

// TODO: Define a function that takes no parameters and returns nothing
func sayHello() {

}

// TODO: Define a function that takes one parameter
func greet(name string) {

}

// TODO: Define a function that returns a value
func add(a, b int) int {

}

// TODO: Define a function with multiple parameters of different types
func createProfile(name string, age int, isActive bool) {

}

func main() {
// TODO: Call the functions you defined above
    sayHello()
    greet("Alice")
    result := add(5, 3)
    fmt.Printf("5 + 3 = %d\n", result)
    createProfile("Bob", 25, true)
}
