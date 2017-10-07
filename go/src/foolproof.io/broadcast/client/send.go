package main

import (
	"flag"
	"log"

	pb "foolproof.io/broadcast/proto"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

const (
	address = "localhost:50051"
)

func main() {
	msg := flag.String("msg", "Hello, World!", "the message to broadcast")
	flag.Parse()

	// Set up a connection to the server.
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewBroadcastClient(conn)

	// Contact the server and print out its response.
	reply, err := c.Broadcast(context.Background(), &pb.BroadcastRequest{Msg: *msg})
	if err != nil {
		log.Fatalf("could not register: %s", err)
	}
	log.Println(reply)
}
