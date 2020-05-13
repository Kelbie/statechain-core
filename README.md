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

Testing
Diagrams
Rust
Actix
RocksDB
Explorer
Github Issues or Trello?

1.  B uses function (1) with userPubkey = B to request serverPubkey A

        let user_priv_key = KeyPair::create();

        server_pub_key, server_ephemeral_key = fetch('/init', user_pub_key);

2.  B then generates transitory key X, and creates a single MuSig key AX (key X is called “transitory” because its private key will later be passed on)

        // https://github.com/KZen-networks/multi-party-schnorr/blob/master/src/protocols/aggsig/test.rs#L26

        // round 0: generate signing keys
        let X = KeyPair::create();

        // round 1: send commitments to ephemeral public keys
        let X_ephemeral_key = EphemeralKey::create();
        let X_commitment = &X_ephemeral_key.commitment;

        // compute aggregate public key:
        let mut pks: Vec<GE> = Vec::new();
        pks.push(X.public_key.clone());
        pks.push(server_public_key);
        let X_key_agg = KeyAgg::key_aggregation_n(&pks, 0);
        let server_key_agg = KeyAgg::key_aggregation_n(&pks, 1);

3)  B prepares tx1: 1BTC to AX (he doesn't send it yet)

        tx1 = transaction.fund(1, AX)

4)  B creates tx2: an eltoo tx [3] that assigns the 1BTC back to B (off-chain)

5.  B uses (2) with nextUserPubkey = B and blindedMessage = tx2

        signed_blinded_message = fetch('/transfer', next_user_pub_key, blinded_message)

        // B completes signature with X
        let is_musig = true;

        let mut pks: Vec<GE> = Vec::new();
        pks.push(X.public_key.clone());
        pks.push(server_pub_key.public_key.clone());
        let X_key_agg = KeyAgg::key_aggregation_n(&pks, 0);

        let X_r_tag = EphemeralKey::add_ephemeral_pub_keys(
            &X_ephemeral_key.keypair.public_key,
            &server_ephemeral_key.keypair.public_key,
        );

        let X_h_0 =
            EphemeralKey::hash_0(&X_r_tag, &X_key_agg.apk, &tx2, is_musig);

        let signed_message = EphemeralKey::sign(
            &X_ephemeral_key,
            &X_h_0,
            &X,
            &X_key_agg.hash,
        );

        let r = X_ephemeral_key.keypair.public_key.x_coor().unwrap();

        let (r, signature) = EphemeralKey::add_signature_parts(signed_message, &signed_blinded_message, &X_r_tag);

        <!-- ! plug signature into transaction -->

6.  B sends tx1 to the blockchain and waits for it to confirm

        await tx1.send()

7.  B receives a key from C in order to prepare a payment

8)  B creates tx3: an eltoo tx (with higher priority) with 1BTC to C (off-chain)

9.  B uses (2) with nextUserPubkey = C and blindedMessage = tx3

        signed_blinded_message = fetch('/transfer', next_user_pub_key, blinded_message)

10) B passes the private key of X (the transitory key) on to C

11) C takes blinded tx2 and tx3 from the public server output and C only accepts the payment if everything is in order [4]
