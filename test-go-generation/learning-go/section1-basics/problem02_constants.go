// Problem: Constants and Iota
// Topic: Constants
// Difficulty: 1

package main

import "fmt"

func main() {
// TODO: Declare a constant named 'pi' with value 3.14159

// TODO: Declare multiple constants in a group
    const (
// TODO: Add constants for days in week, hours in day
    )

// TODO: Use iota to create enumerated constants
    const (
    Sunday = iota
    Monday
    Tuesday
// TODO: Add remaining days
    )

    fmt.Printf("Pi: %f\n", pi)
    fmt.Printf("Days in week: %d, Hours in day: %d\n", daysInWeek, hoursInDay)
    fmt.Printf("Sunday: %d, Monday: %d, Tuesday: %d\n", Sunday, Monday, Tuesday)
}
