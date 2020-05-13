const Buffer = require('safe-buffer').Buffer; 
const BigInteger = require('bigi');
const randomBytes = require('random-bytes');
const randomBuffer = (len) => Buffer.from(randomBytes.sync(len));
const schnorr = require('bip-schnorr');
const convert = schnorr.convert;
const muSig = schnorr.muSig;
const bitcoin = require('bitcoinjs-lib');

const async = require("async");
var request_sync = require('request');

function request(url) {
    return new Promise(function (resolve, reject) {
        request_sync(url, function (error, res, body) {
        if (!error && res.statusCode == 200) {
            resolve(body);
        } else {
            reject(error);
        }
        });
    });
}

async function init(to) {
    var options = {
        'method': 'POST',
        'url': 'http://localhost:9939/init',
        'headers': {
            'Content-Type': ['application/json']
        },
        'body': JSON.stringify({to})
    };


    const response = await request(options)

    return response

}


// 1.  B uses function (1) with userPubkey = B to request serverPubkey A

const userPubKey = Buffer.from('02DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659', 'hex');
const userPrivKey = BigInteger.fromHex('B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF');

// const server_pub_key, server_ephemeral_key = init(userPubKey);

// 2.  B then generates transitory key X, and creates a single MuSig key AX (key X is called “transitory” because its private key will later be passed on)

const xPubKey = Buffer.from('02DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659', 'hex');
const xPrivKey = BigInteger.fromHex('B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF');

const AX = schnorr.muSig.pubKeyCombine([userPubKey, xPubKey]);

// 3)  B prepares tx1: 1BTC to AX (he doesn't send it yet)

const { address } = bitcoin.payments.p2pkh({ network: bitcoin.networks.regtest, pubkey: bitcoin.ECPair.fromPublicKey(AX).publicKey })

var tx = new bitcoin.TransactionBuilder(bitcoin.networks.regtest);

tx.addInput("53fc6d61fa03e2b88f77cc905acccc7a099048cf086dde4ec23ae1f91c71fd0b", 0);
tx.addOutput(address, 100_000_000_000); // 1000 satoshis will be taken as fee.

// 4)  B creates tx2: an eltoo tx [3] that assigns the 1BTC back to B (off-chain)