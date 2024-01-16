# Fe
I really enjoy learning about compilers and tinkering, so I decided to start a
side project to build a mini cmpiler in rust (hence the name Fe, referencing
Iron, since this will run on rust)

## The Formal Grammar
Check out the <a href="https://github.com/spineda2019/Fe/blob/main/Grammar.md">formal grammer</a>
I am working on for the Fe language. It is definitely still a work in progress!

## Motivations
One thing I am aiming to implement in this pet project is having my rudimentary
type system be very explicit, while still being readable. I want to deny
"default" behavior as much as possible, as in my belief, forcing being
explicit improves readabiliy. For example, in trusty C++ (which I love btw) we
have the following behavior:

```cpp
class Vehicle {
    std::uint64_t weight_in_pounds;

    public:
        Vehicle(std::uint64_t weight): weight_in_pounds(weight) {}
}; // Vehicle
```

C++ regulars will recognize that weight_in_pounds is a private class member.
However, someone coming from python might not be able to discern that, because
this is *default* behavior. C++ Enthusiasts will also know that in the case of
structs, this behavior is flipped! Thus, for this mini project I would like to
make a system that roots out default behavior, partly for fun, and partly
because of principle.

When you become extremely pedantic, you can even see this default behavior in
most facets of software. Take the following for example:

```c
int rpm = 30;
```

This is very normal C code, but we already encounter some common ambiguities.
What is the size of the variable "rpm"? We don't know! It could depend on the
target machine. Can rpm be negative? Well, in real life it can't, but in C,
of course an int can be negative! A seasoned engineer would respond to this by
insisting we can use explicit data types (as we did earlier) such as uint64_t.
This assertion is correct, but for this project, why not disallow disambiguity
all together (say for example by having sint32 be a type for signed integers
instead of int)

Consider the following (work in progress) Fe code:

```py
variable speed = 30: sint32;
constant limit = 65: uint8;


class Vehicle {
    public:
        method foo(x: sint64) -> boolean {
            return x > 10;
        }
            
    private:
        method helper(y: uint8) -> uint8 {
            return y + 2;
        }
} # Vehicle
```

TODO

## Compound Operators
Let's define what compound operators we want, as this will unblock our current
peeking task.

```c
-> // For designating return types
+=
-=
*=
/= // Regular compound assignment
```
