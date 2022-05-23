# explain some of the ways hashing functions enable blockchain technology

Hashing function is a function which takes an infinite number of bits, performs calculation on them, and outputs a fixed number of bits. It enables forming a blockchain by linking blocks with previous block hash. It also helps signing & verifying transactions to prevent malicious transactions.

# briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)

The UTXO model is a verification model which allows users to submit txs that specify the results of the state transition, defined as new tx outputs spendable by the receivers. Nodes then verify if the consumed inputs are unspent and if the signatures satisfy the spending conditions.

# what is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)

A bitcoin block is made of a header which contains metadata followed by a list of txs. Block header contains previous block hash, merkle root, timestamp, pow difficulty target, and pow nonce. Previous block hash is what links blocks to form a blockchain. Txs within a block are stored in a merkle tree data structure and we store the merkle root inside the block header.

# what problem/s are POW/POS trying to solve? discuss/compare (byzantine fault tolerance, reaching a single consensus on a p2p network)

POW and POS are algorithms to reach consensus in blockchain networks. In a p2p network, BFT was required in order to reach a common conclusion regarding the order of txs. To archive consensus, network nodes that process the tx needs to agree upon a single version of truth of tx ordering by solving mathematical problem with their computing power(for pow) or by staking digital currency(for pos).
