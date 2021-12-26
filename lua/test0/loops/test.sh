#!/bin/bash
var=1
while true 
do
    var=$(expr $var + 1)
    echo $var
    [ "$var" == "1000000" ] && break 
done

