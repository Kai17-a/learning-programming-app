// Problem: Multiple Return Values
// Topic: Multiple Returns
// Difficulty: 2

package main

import "fmt"

// TODO: Define a function that returns multiple values
func divide(a, b float64) (float64, error) {

}

// TODO: Define a function with named return values
func getNameAndAge() (name string, age int) {

}

// TODO: Define a function that returns multiple values of same type
func getMinMax(numbers []int) (int, int) {

}

// TODO: Define a function that swaps two values
func swap(x, y string) (string, string) {

}

func main() {
// TODO: Call function with multiple returns and handle both values
    result, err := divide(10, 3)
    if err != nil {
        fmt.Printf("Error: %v\n", err)
        } else {
            fmt.Printf("10 / 3 = %f\n", result)
        }

// TODO: Call function with named returns
        name, age := getNameAndAge()
        fmt.Printf("Name: %s, Age: %d\n", name, age)

// TODO: Use blank identifier to ignore return values
        numbers := []int{5, 2, 8, 1, 9}
        min, _ := getMinMax(numbers)  // Ignore max value
        fmt.Printf("Minimum value: %d\n", min)

// TODO: Call swap function
        a, b := "hello", "world"
        a, b = swap(a, b)
        fmt.Printf("After swap: %s, %s\n", a, b)
    }
