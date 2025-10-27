// Problem: Function Parameters and Arguments
// Topic: Function Basics
// Difficulty: 1

package main

import "fmt"

// TODO: Function with value parameters (pass by value)
func modifyValue(x int) {

}

// TODO: Function with pointer parameters (pass by reference)
func modifyPointer(x *int) {

}

// TODO: Function with slice parameter
func modifySlice(numbers []int) {

}

// TODO: Function with map parameter
func modifyMap(data map[string]int) {

}

// TODO: Function with struct parameter
type Person struct {
    Name string
    Age  int
}

func modifyStruct(p Person) {

}

func modifyStructPointer(p *Person) {

}

func main() {
// TODO: Test value vs pointer parameters
    value := 10
    fmt.Printf("Original value: %d\n", value)

    modifyValue(value)
    fmt.Printf("After modifyValue: %d\n", value)

    modifyPointer(&value)
    fmt.Printf("After modifyPointer: %d\n", value)

// TODO: Test slice parameter
    numbers := []int{1, 2, 3, 4, 5}
    fmt.Printf("Original slice: %v\n", numbers)

    modifySlice(numbers)
    fmt.Printf("After modifySlice: %v\n", numbers)

// TODO: Test map parameter
    data := map[string]int{"a": 1, "b": 2}
    fmt.Printf("Original map: %v\n", data)

    modifyMap(data)
    fmt.Printf("After modifyMap: %v\n", data)

// TODO: Test struct parameters
    person := Person{Name: "Alice", Age: 25}
    fmt.Printf("Original struct: %+v\n", person)

    modifyStruct(person)
    fmt.Printf("After modifyStruct: %+v\n", person)

    modifyStructPointer(&person)
    fmt.Printf("After modifyStructPointer: %+v\n", person)
}
