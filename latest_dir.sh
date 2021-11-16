#!/bin/sh
# get the commit message's last element
GITLOG=$(git log -1 --pretty=format:"%s")
LOG_ELEMENTS=' ' read -r -a array <<< "$GITLOG"
directory="${array[-1]}"

# remove the brackets from the last element
directory_name=${directory#"["}
directory_name=${directory_name::-1}
cd $directory_name