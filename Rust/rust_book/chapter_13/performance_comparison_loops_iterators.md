## Notes

One of the most important questions which will help answering the question of whether to use loops or 
iterators is, Which one is faster?

test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

The rust book benchmark the grep project we made with loop vs iter and found that they are essentially the same. 
The iter is ever so slightly slower (literally 2% rounded up)

Largely, idiomatic rust pushes for iterators as the main approach you should consider, at best they might actually
be faster than loops written by people due to allowing people, using abstraction, a much easier time approaching the problem. 
At worst, it is slighty slower than loops normally. 

All in all, I will try to use iterators when it fits the problem, that is, when it is not too awkward to use them. 
