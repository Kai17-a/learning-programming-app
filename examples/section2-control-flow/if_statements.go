package main

import "fmt"

func main() {
    x := 10
    
    if x > 5 {
        fmt.Println("x is greater than 5")
    }
    
    if x%2 == 0 {
        fmt.Println("x is even")
    } else {
        fmt.Println("x is odd")
    }
    
    // If with initialization
    if y := x * 2; y > 15 {
        fmt.Printf("y (%d) is greater than 15\n", y)
    }
}