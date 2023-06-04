import threading
import time
#import copy
#import os


class Stopwatch():
    def __init__(self):
        self.refresh_rate_msec = 1000
        self.counter_rate_msec = 100

        self.refresh_rate_sec = self.refresh_rate_msec / 1000
        self.counter_rate_sec = self.counter_rate_msec / 1000
        
        self.counts_per_sec = 1000 // self.counter_rate_msec
        self.counts_per_min = 60 * self.counts_per_sec
        self.counts_per_hour = 3600 * self.counts_per_sec
        self.count = 0
        self.run = True

        #self.lock = threading.Lock()

    def counter(self):
        while self.run:
            time.sleep(self.counter_rate_sec)
            #self.lock.acquire() # Looks like a good idea to prevent race until another thread acquires a lock first and blocks counter => we loose track of time...
            self.count += 1
            #self.lock.release()

    def refresh(self):
        while self.run:
            #os.system('clear')
            print(self.count_to_time(self.count))
            time.sleep(self.refresh_rate_sec)


    def read_input(self):
        while True:
            resp = input()
            if resp == 'q':
                self.run = False
                break
            elif resp == 'r':
                self.count = 0

    def count_to_time(self, count):
        # count should readily be a pass-by value copy of self.count
        hours = count // self.counts_per_hour
        reminder = count % self.counts_per_hour
        minutes = reminder // self.counts_per_min
        reminder2 = reminder % self.counts_per_min
        seconds = reminder2 // self.counts_per_sec
        milisec = reminder2 % self.counts_per_sec
        return f"{hours}:{minutes}:{seconds}:{milisec}"


if __name__ == "__main__":
    stopwatch=Stopwatch()

    refresh_thread = threading.Thread(target=stopwatch.refresh)
    counter_thread = threading.Thread(target=stopwatch.counter)

    refresh_thread.start()
    counter_thread.start()

    stopwatch.read_input()

    refresh_thread.join()
    counter_thread.join()

