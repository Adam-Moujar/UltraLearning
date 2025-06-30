## Notes

This notes are a discussion on when to return Result and when we should panic!

The decision mostly falls on whether or not you decide that the error should be recoverable or not. 
That is, if it is possible to continue the program, then returning result is more appropiate. 

In prototype code, it makes sense to simply panic if something goes wrong, and then later build more 
robust code. And also in testing, panic might make sense to indicate that a test fails. 

It also might make sense to call unwrap if you are sure that the code should never have an err variant. 
There are cases when the programmer will know that a piece of code shouldn't error, in the case of 
hard coding.

Another case in favour or panic is in code which continuing might be unsafe. 
