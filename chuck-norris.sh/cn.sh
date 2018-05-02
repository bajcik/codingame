#!/bin/bash
# https://www.codingame.com/ide/puzzle/chuck-norris


read MESSAGE
echo "($MESSAGE)" >&2

ENC=$(echo -n "$MESSAGE" \
| od -t o1 \
| sed 's/^[^ ]* *//;' | grep . \
| sed 's/0/000/g; s/1/001/g; s/2/010/g; s/3/011/g; s/4/100/g; s/5/101/g; s/6/110/g; s/7/111/g;
s/^/ /; s/ / 000000/g; s/[^ ]*\([0-9][0-9][0-9][0-9][0-9][0-9][0-9]\)/\1/g; s/ //g;
s/10/1\n0/g; s/01/0\n1/g' \
| while read series; do
    tag=$(echo $series | sed 's/00*/00/; s/11*/0/')
    #block=$(echo $(yes 0 | head -n $(echo -n $series | wc -c))| sed 's/ //g')
    block=$(echo $series|sed 's/1/0/g')
    echo -n "$tag $block "
#    echo -n "$tag:$block " >&2
#    echo "($series->$tag:$block)" >&2
done | sed 's/ $//')

echo $ENC|md5sum >&2
echo $ENC


