package main

import (
	"fmt"
	"net/http"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/handler"
)

// Define a GraphQL schema
var fields = graphql.Fields{
	"hello": &graphql.Field{
		Type: graphql.String,
		Resolve: func(p graphql.ResolveParams) (interface{}, error) {
			return "world", nil
		},
	},
}

var rootQuery = graphql.ObjectConfig{Name: "RootQuery", Fields: fields}
var schemaConfig = graphql.SchemaConfig{Query: graphql.NewObject(rootQuery)}
var schema, _ = graphql.NewSchema(schemaConfig)

func main() {
	// Configure the GraphQL server
	h := handler.New(&handler.Config{
		Schema: &schema,
		Pretty: true,
	})

	// Setup a HTTP server to serve GraphQL requests
	http.Handle("/graphql", h)
	fmt.Println("Server is running on port 8080")
	http.ListenAndServe(":8080", nil)
}
