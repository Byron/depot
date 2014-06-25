// Playground - noun: a place where people can play

import Cocoa

var str = "Hello, playground"
var x: Int = 5

for i in 1..5 {
    var z = sin(10.2)
}

var f = 2
if f < 3 {
    x = 1
} else {
    x = 2
}


var foo = Int.self;

func fun<T> (type: () -> T) -> T {
    let res = type()
    println(res)
    return res;
}

2.advancedBy(5)
2.toIntMax()
Int.max
uint8.max
uint16.max
Int16.max
Int16.min

let (a,b) = (2,3)

a * b
a.advancedBy(4)

fun({2})


enum Signs: Character {
    case Smiley = "😍"
    case Ballon = "🎈"
}

var sign = Signs.Smiley
sign = .Ballon
sign.toRaw()
