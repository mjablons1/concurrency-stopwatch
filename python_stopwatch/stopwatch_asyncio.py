#!/usr/bin/env python3
# stopwatch.py

import asyncio, os

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

    async def counter(self):
        while True:
            await asyncio.sleep(self.counter_rate_sec)
            self.count += 1

    async def refresh(self):
        while True:
            print(self.count_to_time())
            await asyncio.sleep(self.refresh_rate_sec)
            #os.system('clear')

    def read_input(self, loop):
        while True:
            resp = input()
            if resp == 'q':
                all_tasks = asyncio.all_tasks(loop=loop)
                for task in all_tasks:
                    task.cancel()
                break
            elif resp == 'r':
                self.count = 0

    async def start(self):
        loop = asyncio.get_event_loop()
        loop.run_in_executor(None, self.read_input, loop)
        await asyncio.gather(self.counter(), self.refresh())

    def count_to_time(self):
        count = self.count
        hours = count // self.counts_per_hour
        reminder = count % self.counts_per_hour
        minutes = reminder // self.counts_per_min
        reminder2 = reminder % self.counts_per_min
        seconds = reminder2 // self.counts_per_sec
        milisec = reminder2 % self.counts_per_sec
        return f"{hours}:{minutes}:{seconds}:{milisec}"


if __name__ == "__main__":
    stopwatch = Stopwatch()
    try:
        asyncio.run(stopwatch.start())
    except:
        KeyboardInterrupt
        print('Received keyboard interrupt, exiting.')
