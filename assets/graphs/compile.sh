#!/bin/bash

for filename in ./*.dot; do
    extension="${filename##*.}"
    filename="${filename%.*}"
    dot -Tpng "$filename.$extension" > "../images/${filename}.png"
done
