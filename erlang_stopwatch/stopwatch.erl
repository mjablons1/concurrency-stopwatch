-module(stopwatch).
-export([start/0]).
-define(REFRESH_RATE_MSEC, 1000).
-define(COUNTER_RATE_MSEC, 100).
-define(TIMEOUT_MSEC, (?COUNTER_RATE_MSEC * 100)).
-define(COUNTS_PER_SEC, (1000 div ?COUNTER_RATE_MSEC)).
-define(COUNTS_PER_MIN, (60 * ?COUNTS_PER_SEC)).
-define(COUNTS_PER_HOUR, (3600 * (?COUNTS_PER_SEC))).

start() ->
    Counter_PID = spawn(fun() -> counter(0) end),
    spawn(fun() -> refresh(Counter_PID) end),
    read_input(Counter_PID).

counter(Count) ->
    NewCount = receive
                    {tell_count, PID} -> 
                        PID ! {Count, self()},
                        Count;
                    {reset_count, PID} ->
                        PID ! {ok, self()},
                        0
                after
                    0 -> Count
                end,
    sleep_milisec(?COUNTER_RATE_MSEC),
    counter(NewCount+1).

sleep_milisec(Milisec) ->
    receive
        after Milisec -> continue
    end.

read_input(Counter_PID) ->
    case io:get_chars(">", 1) of
        "q" -> init:stop();
        "r" -> Counter_PID ! {reset_count, self()};
        _ -> ignore
    end,
    read_input(Counter_PID).

count_to_time(Count) ->
    Hours = Count div ?COUNTS_PER_HOUR,
    Reminder= Count rem ?COUNTS_PER_HOUR,
    Minutes = Reminder div ?COUNTS_PER_MIN,
    Reminder2 = Reminder rem ?COUNTS_PER_MIN,
    Seconds = Reminder2 div ?COUNTS_PER_SEC,
    Milisec = Reminder2 rem ?COUNTS_PER_SEC,
    lists:concat(lists:join(":",[Hours,Minutes,Seconds,Milisec])).

refresh(PID) ->
    PID ! {tell_count,self()},
    receive
        {Count, PID} -> 
            Time = count_to_time(Count),
            io:format("~s~n",[Time])
    after
        ?TIMEOUT_MSEC -> 
            io:fwrite("Timeout waiting for count from ~p.~n", [PID]),
            init:stop()
    end,
    sleep_milisec(?REFRESH_RATE_MSEC),
    refresh(PID).
