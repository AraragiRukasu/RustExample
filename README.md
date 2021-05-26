# RustExample
This repository holds a simple solution to a simple problem. We have an array and we want to move its elements a certain number of positions. The solutions per se is not the point here, just trying out some features in Rust as an excercise. I'm also aware that the solution is not optimal, again that's not the reason for this repo.

I tried playing around with:
- References and mutability
- Data structures: arrays and vectors
- Modules and crate organization
- Automated Tests
- Console application arguments
- File writing
- High level patterns implementation: simplified Strategy Pattern

Obviously this implementation doesn't makes sense more than to force myself to use certain features and see how to use the language to do things that I'm already used in other platforms.

## Usage
First of all we take two arguments, the number of elements that the array will have and the positions to perform the movement. This problem could take any array as input but that was not a concern so the number of elements is used to generate an array populated with a range of consecutive numbers.

This means, if number of elements is 5 we will use the following array to operate

```
[0, 1, 2, 3, 4]
```

THen, if 2 is the number of positions to move we will get the following result

```
[2, 3, 4, 0, 1]
```

This output will be shown in console if the number of elements on the array is less than or equal to 500, else we will be writing the results into a file on the root of the repo called *output.txt*

So, to run this just execute the crate passing two numbers as arguments, first the number of elements and then the positions to move.
```
cargo run 500 12
```

