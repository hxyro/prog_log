#!/bin/lua
var = true
num = 1
while var do
print(num)
num = num + 1
    if num == 1000000 then
        var = false
    end
end
