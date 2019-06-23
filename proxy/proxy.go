package main

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"math/rand"
	"net"
	"net/http"
	"os"
	"time"
)
import "log"
var FluAddr = "localhost:30000"
var Session string
var Serial = 0
var AppId = 10


type FluResp struct {
	Result struct {
		Data string `json:"data"`
	} `json:"result"`
}

type H struct {
	S *net.Conn
}

func (h *H)ServeHTTP(w http.ResponseWriter, r *http.Request)  {
	fmt.Println("Incoming Req")
	request := &bytes.Buffer{}
	r.Write(request)

	url := fmt.Sprintf("http://%s/apps/%d/tx", FluAddr, AppId)
	tosend:=bytes.NewBufferString(fmt.Sprintf("%s/%d\n", Session,Serial))
	Serial++
	request.WriteTo(tosend)
	//fmt.Println(tosend.String())
	resp, err := http.Post(url, "application/x-www-form-urlencoded", tosend)
	if err != nil {
		w.Write([]byte("Error: " + err.Error()))
		return
	}

	fluresp := FluResp{}
	err = json.NewDecoder(resp.Body).Decode(&fluresp)
	resp.Body.Close()
	if err != nil {
		errstr := fmt.Sprintf("Unmarshal Error, %s", err)
		w.Write([]byte(errstr))
		return
	}

	decoded, err := base64.StdEncoding.DecodeString(fluresp.Result.Data)
	if err != nil {
		errstr := fmt.Sprintf("Decode Error, %s", err)
		w.Write([]byte(errstr))
		return
	}
	W := w.(http.Hijacker)
	conn, _, err := W.Hijack()
	os.Stdout.Write(decoded)
	conn.Write(decoded)
	conn.Close()
}


func main() {
	fmt.Println("Serving ... ")
	Session = fmt.Sprintf("%d", rand.Int31())


	h := H{}
	fmt.Print(http.ListenAndServe(":8001", &h))
}













func serveTcp() {

	addr, _ := net.ResolveTCPAddr("tcp", "localhost:30001")
	listener, err := net.ListenTCP("tcp", addr)
	if err != nil {
		panic("Can't LISTEN")
	}

	for {
		conn, err := listener.AcceptTCP()
		if err != nil {
			panic(err)
		}
		go serve(conn)
	}


}

func serve(conn *net.TCPConn) {
	addr, _ := net.ResolveTCPAddr("tcp", FluAddr)
	upstream, err := net.DialTCP("tcp", nil, addr)
	if err != nil {
		log.Print("Can't connect")
		return
	}

	donedown := make(chan bool)
	doneup := make(chan bool)
	go func() {
		<- donedown
		<- doneup
		conn.Close()
		upstream.Close()
	}()

	Serial ++
	req := fmt.Sprintf("POST /apps/%d/tx HTTP/1.0\nHost:%s\nTransfer-Encoding: chunked\n\n", AppId, FluAddr)
	fmt.Printf("SEND: %s\n", req)
	upstream.Write([]byte(req))

	sig := fmt.Sprintf("sessionId%s/%d\n", Session, Serial)
	S := fmt.Sprintf("%d\n", len(sig))
	upstream.Write([]byte(S))
	upstream.Write([]byte(sig))

	go func() {
		buff := make([]byte, 4096)
		for {
			upstream.SetReadDeadline(time.Now().Add(time.Second))
			s, err := upstream.Read(buff)
			if e, ok := err.(interface{ Timeout() bool }); ok && e.Timeout() {
			} else if err != nil {
				fmt.Printf("%#v", err)
				log.Print("Can't read upstream", err)
				doneup <- true
				return
			}
			if s == 0 {
				continue
			}
			conn.Write(buff[:s])
			fmt.Println("<<< ", buff)

		}
	}()

	buff := make([]byte, 4096)
	for {
		conn.SetReadDeadline(time.Now().Add(time.Second))
		size, err := conn.Read(buff)
		if e, ok := err.(interface{ Timeout() bool }); ok && e.Timeout() {
		} else if err != nil {
			log.Print("Can't read conn", err)
			upstream.Write([]byte("0\n"))
			donedown <- true
			return
		}
		if size == 0 {
			continue
		}
		sizes := fmt.Sprintf("%d\n", size)
		upstream.Write([]byte(sizes))
		upstream.Write(buff[:size])
		fmt.Println(">>> ", buff)

	}
}
