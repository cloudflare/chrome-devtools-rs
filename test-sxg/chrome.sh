#!/bin/bash
echo "hello"
input="./invalid.csv"
n=0

while IFS= read -r url
do
    if [ $n \< 50 ] ; then
        open -a /Applications/Google\ Chrome.app $url
    fi
    n=$((n + 1))
done < "$input"
