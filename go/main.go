package main

import (
	"fmt"
)

// Sync benchmarks a synchronous channel
func Sync(n int) int {
	c := make(chan int)
	go func(c chan int, n int) {
		for i := 0; i < n; i++ {
			c <- i
		}
	}(c, n)

	a := 0
	for i := 0; i < n; i++ {
		a += <-c
	}
	return a
}

// Async bencmarks an asynchronous channel
func Async(n int) int {
	c := make(chan int, int(n))
	go func(c chan int, n int) {
		for i := 0; i < n; i++ {
			c <- 1
		}
	}(c, n)
	b := 0
	for i := 0; i < n; i++ {
		b += <-c
	}
	return b
}

func main() {
	fmt.Println("ok")
}
