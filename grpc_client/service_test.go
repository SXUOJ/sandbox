package main

import (
	"context"
	"fmt"
	"grpc_client/pb"
	"log"
	"testing"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func TestPing(t *testing.T) {
	conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewJudgerClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	r, err := c.Ping(ctx, &pb.PingRequest{})

	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}
	fmt.Println(r)
}

func TestHello(t *testing.T) {
	conn, err := grpc.Dial(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewJudgerClient(conn)

	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	r, err := c.Judge(ctx, &pb.JudgeRequest{
		Type: "C",
		Source: `#include<stdio.h>
int main(){
	printf("Hello\n");
	return 0;
}
		`,
		Input:       []string{"input"},
		Output:      []string{"output"},
		TimeLimit:   0,
		MemoryLimit: 0},
	)

	if err != nil {
		log.Fatalf("could not greet: %v", err)
	}
	fmt.Println(r)

}
