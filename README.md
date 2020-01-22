# statechain-core

- Needs to be able to lookup money earned with paypercall
- init & transfer endpoints should require paypercall payments
- https://www.influxdata.com/blog/benchmarking-leveldb-vs-rocksdb-vs-hyperleveldb-vs-lmdb-performance-for-influxdb/
- https://bitcoin.stackexchange.com/questions/28168/what-are-the-keys-used-in-the-blockchain-leveldb-ie-what-are-the-keyvalue-pair


<table>
    <thead>
        <tr>
            <td></td>
            <th>Key</th>
            <th>Value</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <th>Statechain</th>
            <td>'s' + xx-byte server public key</td>
            <td>
                - info about federation members that signed
            </td>
        </tr>
        <tr>
            <th>Transfer</th>
            <td>'t' + 32-byte transaction hash</td>
        </tr>
        <tr>
            <th>Final Transfer</th>
            <td>'f' + 32-byte transaction hash</td>
        </tr>
    </tbody>
</table>

# TCP Server

No encryption is needed because everything is public information.

TCP was used instead of UDP because we want all the members in the federation to get the newest state information from their peers without uncertainty if the message was successful.

`GetHeaders`, `GetData`, `Inv`

## Connect to TCP Server

    nc 127.0.0.1 9939

# Why Rust?

Safety, Speed, and Concurrency