package main

import "fmt"

func main() {
    // Basic for loop
    fmt.Println("Basic for loop:")
    for i := 0; i < 5; i++ {
        fmt.Printf("Count: %d\n", i)
    }
    
    // While-style loop
    fmt.Println("\nWhile-style loop:")
    j := 0
    for j < 3 {
        fmt.Printf("J: %d\n", j)
        j++
    }
    
    // Range over slice
    fmt.Println("\nRange over slice:")
    numbers := []int{1, 2, 3, 4, 5}
    for index, value := range numbers {
        fmt.Printf("Index: %d, Value: %d\n", index, value)
    }
}