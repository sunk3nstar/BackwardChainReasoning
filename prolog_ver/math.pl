leq(0,3).
leq(7,9).
leq(X,add(X,0)).
leq(add(X,0),X).
leq(X,Z) :- leq(X,Y), leq(Y,Z).
leq(add(W,X),add(Y,Z)) :- leq(W,Y),leq(X,Z).
leq(X,X).
leq(add(X,Y),add(Y,X)).


% Benchmarking
benchmark :-
    writeln('Starting benchmark...'),
    statistics(runtime, [Start|_]),
    call_with_depth_limit(leq(7,add(3,9)),5,Result),
    call_with_depth_limit(leq(7,add(3,9)),5,Result),
    call_with_depth_limit(leq(7,add(3,9)),5,Result),
    call_with_depth_limit(leq(7,add(3,9)),5,Result),
    call_with_depth_limit(leq(7,add(3,9)),5,Result),
    statistics(runtime, [End|_]),
    Runtime is End - Start,
    format('Runtime: ~d ms~n', [Runtime]),
    format('Result: ~w~n', [Result]).

:- benchmark.
:- halt.
