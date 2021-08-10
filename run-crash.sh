#!/bin/bash

if [ -z "$1" ]
then
    echo "The committee size is missing, usage:

    ./run_nodes.sh SIZE

where 2 <= SIZE <= 8"
    exit
fi

killall -9 aleph-node

set -e

clear

echo "$1" > /tmp/n_members

authorities=(Damian Tomasz Zbyszko Hansu Adam Matt Antoni Michal)
authorities=("${authorities[@]::$1}")

echo "[]" > /tmp/fork_history

./target/debug/aleph-node dev-keys  --base-path /tmp --chain dev --key-types aura alp0

for i in ${!authorities[@]}; do
  auth=${authorities[$i]}

  ./target/debug/aleph-node purge-chain --base-path /tmp/$auth --chain dev -y
  ./target/debug/aleph-node \
    --validator \
    --chain dev \
    --base-path /tmp/$auth \
    --name $auth \
    --ws-port $(expr 9944 + $i) \
    --rpc-port $(expr 9933 + $i) \
    --port $(expr 30334 + $i) \
    --execution Native \
    -lafa=trace \
    -lAlephBFT=trace \
    2> run1-$auth-$i.log > out& \
done


echo "Sleeping"
sleep 40
echo "Killing and restarting"

killall -9 aleph-node

authorities=(Damian Tomasz Zbyszko Hansu Adam Matt Antoni Michal)
authorities=("${authorities[@]::$1}")

echo "[{\"block_num\":10,\"bad_blocks\":[]}]" > /tmp/fork_history

for i in ${!authorities[@]}; do
  auth=${authorities[$i]}
  ./target/debug/aleph-node \
    --validator \
    --chain dev \
    --base-path /tmp/$auth \
    --name $auth \
    --ws-port $(expr 9944 + $i) \
    --rpc-port $(expr 9933 + $i) \
    --port $(expr 30334 + $i) \
    --execution Native \
    -lafa=trace \
    -lAlephBFT=trace \
    2> run2-$auth-$i.log > out& \
done


