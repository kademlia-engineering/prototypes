package main

import (
	"fmt"
	"net/http"
)

func main() {
	// Define a handler function for the root ("/") route
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "Hello, World!")
	})

	// Start the web server on port 8080
	port := 8080
	fmt.Printf("Server is running on port %d...\n", port)
	err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil)
	if err != nil {
		fmt.Println("Error:", err)
	}
}
