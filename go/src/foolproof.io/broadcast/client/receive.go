package main

import (
	"flag"
	"log"
	"time"

	pb "foolproof.io/broadcast/proto"
	"golang.org/x/net/context"
	"google.golang.org/grpc"
)

const (
	address = "localhost:50051"
)

func main() {
	key := flag.String("key", "default_key", "The key to register under")
	flag.Parse()
	if flag.NArg() != 1 {
		log.Fatal("pass in an argument")
	}

	// Set up a connection to the server.
	conn, err := grpc.Dial(address, grpc.WithInsecure())
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()
	c := pb.NewPresenceClient(conn)

	// Contact the server and print out its response.
	switch flag.Arg(0) {
	case "register":
		stream, err := c.Register(context.Background())
		if err != nil {
			log.Fatalf("%v.Register(_) = _, %s", c, err)
		}
		for {
			if err := stream.Send(&pb.Heartbeat{Key: *key}); err != nil {
				log.Fatalf("%v.Send() = %s", stream, err)
			}
			time.Sleep(1 * time.Second)
		}
		reply, err := stream.CloseAndRecv()
		if err != nil {
			log.Fatalf("%v.CloseAndRecv() got error %v, want %v", stream, err, nil)
		}
		log.Println(reply)
	case "list":
		reply, err := c.List(context.Background(), &pb.ListRequest{})
		if err != nil {
			log.Fatalf("could not register: %s", err)
		}
		log.Println(reply)
	case "monitor":
		reply, err := c.Monitor(context.Background(), &pb.MonitorRequest{})
		if err != nil {
			log.Fatalf("could not register: %s", err)
		}
		log.Println(reply)
	}
	log.Printf("done :)")
}
