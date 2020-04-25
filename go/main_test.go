package main

import "testing"

func BenchmarkSync(b *testing.B) {
	Sync(1000)
	// Sync(10000)
}

func BenchmarkAsync(b *testing.B) {
	Async(1000)
	// Async(10000)
}
