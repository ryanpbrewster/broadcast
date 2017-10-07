package main

import (
	"log"

	pb "foolproof.io/broadcast/proto"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

const (
	address = "localhost:50051"
)

func main() {
	// Set up a connection to the server.
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewBroadcastClient(conn)

	// Contact the server and print out its response.
	stream, err := c.Listen(context.Background(), &pb.ListenRequest{})
	if err != nil {
		log.Fatalf("could not listen: %s", err)
	}
	for {
		evt, err := stream.Recv()
		if err != nil {
			log.Fatalf("failure while listening: %s", err)
		}
		log.Println(evt)
	}
}
