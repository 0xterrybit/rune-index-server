
1. install bitcoin
   https://gist.github.com/rjmacarthy/b56497a81a6497bfabb1

2. install ordinals
   https://www.wavelayer.com/blog/how-to-install-and-run-a-bitcoin-node-on-ubuntu-22-04/


ord --bitcoin-rpc-username=bitcoin  --bitcoin-rpc-password=bitcoin  --bitcoin-data-dir=/mnt/data/.bitcoin --bitcoin-rpc-url=https://127.0.0.1:8332 --chain=testnet  --data-dir=/mnt/data/.ord/  --index=/mnt/data/.ord/index.redb --index-runes runes



ord --bitcoin-rpc-username bitcoin  --bitcoin-rpc-password bitcoin  --bitcoin-rpc-url=https://127.0.0.1:8332 --chain=testnet --index=/mnt/data/.ord/index.redb --index-runes --server-url http://localhost:8080 server



  wallet balance

ord --config=/mnt/data/.ord/ord.yaml --config-dir


rpcuser=bitcoin
rpcpassword=bitcoin
testnet=1

rpcallowip=0.0.0.0/0
rest=1
server=1
txindex=1
datadir=/mnt/data/.bitcoin
[test]
rpcport=8080

bitcoind  -conf=bitcoin.conf --daemon
bitcoin-cli  -conf=.bitcoin/bitcoin.conf getblockcount


ps -e | grep bitcoin 
kill -9

bitcoind --daemon
bitcoin-cli getblockcount

deprecatedrpc=accounts
addnode=119.23.67.156
addnode=47.224.175.1
addnode=39.105.39.182
addnode=120.24.70.214
addnode=39.100.228.213
addnode=43.226.37.242
addnode=121.18.238.39
addnode=42.59.56.174



https://www.cnblogs.com/coinbt/p/8325157.html
https://pulsedive.com/ioc/testnet-seed.bitcoin.jonasschnelli.ch




ord --config=/mnt/data/.ord/ord.yaml server --decompress  --csp-origin=http://35.197.131.125:80

ord --config=/mnt/data/.ord/ord.yaml server --decompress  --csp-origin=http://35.197.131.125:80
