// Problem: Recursive Functions
// Topic: Function Basics
// Difficulty: 2

package main

import "fmt"

// TODO: Implement factorial using recursion
func factorial(n int) int {

}

// TODO: Implement Fibonacci using recursion
func fibonacci(n int) int {

}

// TODO: Implement sum of digits using recursion
func sumDigits(n int) int {

}

// TODO: Implement binary search using recursion
func binarySearch(arr []int, target, left, right int) int {

}

func main() {
// TODO: Test factorial
    for i := 0; i <= 5; i++ {
        fmt.Printf("Factorial of %d: %d\n", i, factorial(i))
    }

// TODO: Test Fibonacci
    fmt.Println("Fibonacci sequence:")
    for i := 0; i < 10; i++ {
        fmt.Printf("F(%d) = %d\n", i, fibonacci(i))
    }

// TODO: Test sum of digits
    numbers := []int{123, 456, 789}
    for _, num := range numbers {
        fmt.Printf("Sum of digits in %d: %d\n", num, sumDigits(num))
    }

// TODO: Test binary search
    sortedArray := []int{1, 3, 5, 7, 9, 11, 13, 15}
    target := 7
    index := binarySearch(sortedArray, target, 0, len(sortedArray)-1)

    if index != -1 {
        fmt.Printf("Found %d at index %d\n", target, index)
        } else {
            fmt.Printf("%d not found in array\n", target)
        }
    }
