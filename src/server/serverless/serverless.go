package main

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handler(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	// Create a response body
	responseBody := map[string]string{
		"message": "Serverless function executed successfully",
	}

	// Marshal response body to JSON
	responseJSON, err := json.Marshal(responseBody)
	if err != nil {
		return events.APIGatewayProxyResponse{}, err
	}

	// Return HTTP response
	return events.APIGatewayProxyResponse{
		StatusCode: 200,
		Headers: map[string]string{
			"Content-Type": "application/json",
		},
		Body: string(responseJSON),
	}, nil
}

func main() {
	lambda.Start(handler)
}
