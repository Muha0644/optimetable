# Timetable Optimizer program
I made this little thing to learn Rust, but also so i can spend as little time at university as possible.

Optimetable first generates every single valid timetable using a bruteforce depth-first search,
and then filters the results out based on certain criteria _(in this case minimizing empty time between classes, and also blank days if possible)_
It returns an array with possible solutions of equal efficiency.

The program assumes that each subject has a seperate "lecture" and "lab" class which can be at one or more times, and only one of each has to be attended.
