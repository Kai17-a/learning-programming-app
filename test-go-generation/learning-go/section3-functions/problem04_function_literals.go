// Problem: Function Literals and Anonymous Functions
// Topic: Variadic Functions
// Difficulty: 3

package main

import "fmt"

func main() {
// TODO: Define an anonymous function and call it immediately
    func() {
        fmt.Println("This is an anonymous function")
    }()

// TODO: Assign a function literal to a variable
    multiply := func(a, b int) int {
        return a * b
    }

    result := multiply(4, 5)
    fmt.Printf("4 * 5 = %d\n", result)

// TODO: Define a function that returns a function
    makeAdder := func(x int) func(int) int {
        return func(y int) int {
            return x + y
        }
    }

    add5 := makeAdder(5)
    fmt.Printf("5 + 3 = %d\n", add5(3))

// TODO: Use function as parameter
    numbers := []int{1, 2, 3, 4, 5}

    applyOperation := func(nums []int, op func(int) int) []int {
        result := make([]int, len(nums))
        for i, num := range nums {
            result[i] = op(num)
        }
        return result
    }

// Square each number
    squared := applyOperation(numbers, func(x int) int {
        return x * x
    })

    fmt.Printf("Original: %v\n", numbers)
    fmt.Printf("Squared: %v\n", squared)
}
