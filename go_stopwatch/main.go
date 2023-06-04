package main

import (
	"fmt"
	"time"
  // "os"
  // "os/exec"
  )

var COUNTER_RATE_MSEC int = 100
var REFRESH_RATE_MSEC int = 1000

var COUNTS_PER_SEC int = 1000 / COUNTER_RATE_MSEC
var COUNTS_PER_MIN int = 60 * COUNTS_PER_SEC
var COUNTS_PER_HOUR int = 3600 * COUNTS_PER_SEC
var TIMEOUT int = COUNTER_RATE_MSEC * 2

func counter(request <-chan string, answer chan<- int) {
	var count int = 0

  for count = 0; ; count++ { 
    select {
      case msg := <- request: // we don't block execution here only because the "default: break" statement (breaks out of the switch statement unless the request is immediately available)
          if msg == "get_count"{
            answer <- count  // we don't block execution here only because we have added a buffer on answer channel definition
          } else if msg == "reset_count"{
            count = 0
          }
      default:
        break
    }
    time.Sleep(time.Millisecond * time.Duration(COUNTER_RATE_MSEC))
  }
}

func refresh(request chan<- string, answer <-chan int) {

  for {
    request <- "get_count" // we don't want to get blocked here either because this way we will get no chance to handle timeout, hence we buffer the request channel as well
    // time.Sleep(REFRESH_RATE_SEC) // moving sleep to this line is a great way to illustrate how having or not having a buffered channel influences counting due to default blocking behavior
    select{
      case count := <- answer:
        // cmd := exec.Command("clear") //This works but only on nix, is not particularly good practice and adds code.
        // cmd.Stdout = os.Stdout
        // cmd.Run()
        fmt.Println(count_to_time(count))
      case <- time.After(time.Millisecond * time.Duration(TIMEOUT)):
        fmt.Println("Timeout waiting for count.")
    }
  time.Sleep(time.Millisecond * time.Duration(REFRESH_RATE_MSEC))
  }
}

func read_input(request chan<- string) {
  var input string
  for {
    fmt.Scanln(&input)
    if input == "r"{
      request <- "reset_count"
    } else if input == "q" {
      break
    }
  }

}

func count_to_time(count int) string {
  // by default go does pass-by-value so count is not volatile here
  hours := count / COUNTS_PER_HOUR
  reminder := count % COUNTS_PER_HOUR
  minutes := reminder / COUNTS_PER_MIN
  reminder = reminder % COUNTS_PER_MIN
  seconds := reminder / COUNTS_PER_SEC
  milisec := reminder % COUNTS_PER_SEC
  var time string = fmt.Sprintf("%v:%v:%v:%v", hours, minutes, seconds, milisec)
  return time
}

func main() {
  request := make(chan string, 1)
  answer := make(chan int, 1)
  go refresh(request, answer) // refresh routine needs request channel to send and answer to receive
  go counter(request, answer) // counter routine needs request channel to receive and answer to send
  read_input(request)
}

