#!/bin/bash

for num in {0..3}
do
	gnome-terminal -t "STOPWATCH${num}"  -- erl -eval 'stopwatch:start().'

done

