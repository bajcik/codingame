#!/bin/bash

read expression                                             # 10 * ? = 70

left=$(echo "$expression" | sed 's/=.*//; s/ //g')          # 10*?
result=$(echo "$expression" | sed 's/.*= //')               # 70
pattern=$(echo $left | sed 's/?/{0,1,2,3,4,5,6,7,8,9}/g')   # 10*{0,1,2,3,4,5,6,7,8,9}
echo "@ $expression @ $left @ $result @ $pattern @" >&2

# brute-force
for ex in $(sh -c "echo $pattern"); do
	exresult=$(echo "$ex" | bc -l | sed 's/.00000000000000000000//')
	[ "$exresult" == "$result" ] && echo "$ex" | sed "s/[-+/*]/ & /g; s/$/ = $result/"
	#echo " ex=$ex = exresult=$exresult" >&2
done


