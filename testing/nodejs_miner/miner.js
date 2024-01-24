const axios = require("axios");
const sha256 = require("js-sha256").sha256;

const NODE_URL = "http://localhost:8000";
const DIFFICULTY = 14; // number of zeros needed to prefix hash (bits)

async function getLatestBlock() {
    const response = await axios.get(`${NODE_URL}/frontier_block`);
    return response.data;
}

function byteToBinary(byte) {
    return ('00000000' + byte.toString(2)).slice(-8);
}

function isWorkValid(hashBuffer) {
    const bitsArray = Array.from(new Uint8Array(hashBuffer), byteToBinary);
    const bitsString = bitsArray.join('');
    let i = 0;
    while (i < DIFFICULTY) {
        if (bitsString[i] != '0') return false;
        i++;
    };
    return true;
}

async function publishBlock(block) {
    const response = await axios.post(`${NODE_URL}/publish_block`, block);
    console.log(response.data);
}

async function mineNewBlock(data) {
    getLatestBlock().then(lb => {
        let block = {
            "index": lb.index+1,
            "timestamp": lb.timestamp+1,
            "data": data,
            "previous": lb.hash,
            "nonce": 0,
            "hash": "0000000000000000000000000000000000000000000000000000000000000000"
        };
    
        let dataHash = sha256.create();
        dataHash.update(block.data);
        let blockHash = sha256.create();
    
        do {
            // console.log(`${block.index}${block.timestamp}${dataHash.hex()}${block.previous}${block.nonce}`);
            block.nonce += 1;
            blockHash = sha256.create();
            blockHash.update(`${block.index}${block.timestamp}${dataHash.hex()}${block.previous}${block.nonce}`);
            block.hash = blockHash.hex();
            // console.log(blockHash.hex());
        } while (!isWorkValid(blockHash.arrayBuffer()));
            
        publishBlock(block);
        mineNewBlock(`${Math.random()} is random!`);
    })
}

mineNewBlock(`${Math.random()} is random!`);