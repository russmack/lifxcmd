#!/bin/sh

# Author: Russell Mackenzie
# Date: 2017

while [ 1 ]; do
    H=`date "+%H"`

    case $H in
        "20") echo "is 20"
            lifxcmd -h=51 -s=84 -b=100
            ;;
        "21") echo "is 21"
            lifxcmd -h=51 -s=84 -b=77
            ;;
        "22") echo "is 22"
            lifxcmd -h=36 -s=100 -b=74
            ;;
        "23") echo "is 23"
            lifxcmd -h=36 -s=100 -b=50
            ;;
        "00") echo "is 24"
            lifxcmd -h=36 -s=100 -b=35
            ;;
    esac

    sleep 300

done

