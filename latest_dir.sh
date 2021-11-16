#!/bin/bash
# get the commit message's last element
GITLOG=$(git log -1 --pretty=format:"%s")
echo "LAST COMMIT MESSAGE: ${GITLOG}"
LOG_ELEMENTS=' ' read -r -a array <<< "$GITLOG"
directory="${array[-1]}"

# remove the brackets from the last element
directory_name=${directory#"["}
directory_name=${directory_name::-1}
echo "USED DIRECTORY FOR BUILD: ${directory_name}"
cd $directory_name

# after changing to latest directory build software
echo "BUILDING SW"
cargo build --verbose
echo "RUNNING SW"
cargo run
