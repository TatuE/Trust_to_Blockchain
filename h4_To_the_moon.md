# h4 To the moon!

## Assignment

Full assignment can be viewed at the courses website : [h4 To the moon!](https://terokarvinen.com/trust-to-blockchain/#h4-to-the-moon)

## Assignment answers

As with the previous assignment, all assignments in this report that require a computer and testing are performed by using my own PC.

### System environment

- Computer
  - CPU: AMD 7700 (Clocked @ 5.45 GHz)
  - Motherboard: ASUS B650M-plus-wifi
  - Memory: 32 GB DDR5 6000MHz
  - Graphics card: AMD RX 7900XTX
  - Storage: 1TB Western digital black SN850X (m.2, nvme)
  - Operating system: EndeavourOS (Arch linux)
  - Desktop: KDE plasma 6 (Wayland)
  - Package manager: pacman

### Assignment links

Links to assignment answers.

- [Answer to assignment x](h4_To_the_moon.md#x)
- [Answer to assignment a](h4_To_the_moon.md#a)
- [Answer to assignment b](h4_To_the_moon.md#b)
- [Answer to assignment c](h4_To_the_moon.md#c)
- [Answer to assignment d](h4_To_the_moon.md#d)
- [Answer to assignment e](h4_To_the_moon.md#e)
- [Answer to assignment f](h4_To_the_moon.md#f)
- [Answer to assignment g](h4_To_the_moon.md#g)

## x

>x) Read and summarize (with some bullet points)
>
>- Nakamoto 2008: Bitcoin: [A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf) [(A colored HTML version)](https://git.dhimmel.com/bitcoin-whitepaper/), chapters
>   - 1 Introduction
>   - 2 Transactions
>   - 3 Timestamp Server
>   - 4 Proof-of-Work
>   - 5 Network
>   - 6 Incentive

### Nakamoto 2008: Bitcoin: A Peer-to-Peer Electronic Cash System

The document summarized can be read at the bitcoin.org website : [Nakamoto 2008: Bitcoin: A Peer-to-Peer Electronic Cash System](https://bitcoin.org/bitcoin.pdf)

The document details the foundation that is used with bitcoin.

As stated in the introduction, modern digital payments have an underlying weakness, they mostly rely on financial institutions to process these transactions and they are based on trust between the payer and payee.
The payer can default or refuse on the payment to the payee and the financial institutions handling this transaction work as mediaries on these disputes. This leeds to increases in transaction costs and an underlying distrust between parties.

The paper suggest a a solution, where two parties could perform a transaction without the need for third party intermediaries in a way electronic payment system that is based on cryptographic proof of work, not trust.

The proposed electronic coin is based on a chain of digital signature, each coin has a digitally signed hash of the previous transaction and the public key of the new owner for the said coin. This forms a chain, in which every transaction can be verified back within the chain, note that this verification is based on signatures and public keys, which can provide anonymity for the payer and payee. There is a danger of double spending coins, but the proposed solution is that:

- All transactions must be publicly announced.
- Transactions must be timestamped.
- The majority of users agree that a given transaction has taken place in a given time.

For this to work, a timestamp server is needed. The time stamps server generates a hash of a block, to proof that it has existed in a given time, this timestamp will include also the previous timestamp, thus again, forming a chain that re-enforces it when it grows.

To determine the majority decision, there needs to be proof of work. This is proposed by calculating the a cryptographic puzzle. To demonstrate proof of work, block requires a hash (such as SHA256) that has a predetermined amount of leading zeros in it. The puzzle (or lottery) is to find a nonce (a random or semi-random number if I'm correct) that when hashed, has the required amount of leading zeros in it. A new block will include the hash of the previous block and thus the blocks start to form a chain. This process is referred as CPU power, one-CPU-one-vote. So who makes the up the majority? These are called nodes, essentially a node is a computer connected to the electronic coin network, that are doing the work.

How this works is as follows:

1. A new transaction broadcast.
2. The transaction is collected to a block by the nodes (or more precisely a new block to the chain).
3. The nodes start doing the proof-of-work.
4. When the proof-of-work is completed, the accomplished node broadcasts the block to the network.
5. The other nodes will only accept this block if all transactions in it are deemed valid.
6. The nodes express their acceptance of the new block by creating a new block in the chain, using the previous blocks hash in it.

The idea is, that the longest block is considered the correct one, and nodes keep working to extend it. This makes altering the chain exceedingly hard, since to forge a transaction, one would have to recalculate the chain, or but plainly, redo the proof of work.

There also needs to has to be incentive for doing this and keeping the chain honest (unaltered), there is always the danger that anyone controlling a majority of nodes (CPU power) could try to alter the chain. To provide incentive for keeping the process honest, the first transaction in a block provides a coin, given to the creator of that block. The document makes a reference to gold miners, they do the work for increasing gold in circulation, profiting in the process.

Even though Bitcoin has been around for a "long" time, I'm quite new to the fundamentals of it, honestly for some reason I'm was struggling while reading this document, therefore one could say that it was a much needed reading.

## a

>a) Wallet. Create a BitCoin testnet wallet. (For example, electrum)

### Sources: 
- [Electrum Documentation](https://electrum.readthedocs.io/en/latest/)
  - I started to read this, but in the end, the practical answers I found by trial and error (not that many errors actually).

Now Bitcoin (and cryptocurrency in general) is something that I've been interested in, but I have never used, bought or mined it. I've used [XMRig](https://xmrig.com/) for benchmarking but not mining.

Since I'm a bit of a newbie to this, I'll try Tero's suggestion and install electrum for this.

Let's update the system and install it.

| Electrum install                          |                                               |
| ----------------------------------------- | --------------------------------------------- |
| ![](/img/electrum_installation-1.png)     | ![](/img/electrum_installation-2.png)         |

After the installation, let's start the program.

![](/img/electrum_installation-3.png)

Since it seems that the program is working, let's create a wallet. The process is pretty straight forward, just read the on screen instructions.

| Create a new wallet                   | What kind of wallet                   | Create a new seed                             | Review and save new seed              |
| ------------------------------------- | -----------------------------------   | --------------------------------------------- | ------------------------------------- |
| ![](/img/electrum_installation-4.png) | ![](/img/electrum_installation-5.png) | ![](/img/electrum_installation-6.png)         | ![](/img/electrum_installation-7.png) |

| Confirm new seed                      | Add a password for the wallet         | New wallet created                            |
| ------------------------------------- | -----------------------------------   | --------------------------------------------- |
| ![](/img/electrum_installation-8.png) | ![](/img/electrum_installation-9.png) | ![](/img/electrum_installation-10.png)        |

Since I don't trust post-it-notes, I saved the wallet seed and password to the password manager created in [h2]().

![](/img/electrum_installation-password.png)

### Personal note

The Gui of Electrum in Arch linux is a bit buggy, it seems that Wayland is to blame.  
**Note to self:** After this course, reinstall the system and use i3 or xfce fot the desktop environment, since by recollection, they still use [Xorg](https://wiki.archlinux.org/title/Xorg).

## b

>b) Faucet. Get worthless fake money from a testnet Bitcoin faucet.

It seems that in the previous part I forgot to start Electrum to used with testnet.  
This can be done by adding the *--testnet* parameter while starting the program

- *electrum --testnet*

I intended to use the same wallet as before, but it seems that this will not work, so I will have to create a new wallet.  
No documentation added, since the process is the same as with [assignment a](h4_To_the_moon.md#a).  
The only difference is that the program kindly reminds you that you are in testnet mode.

![](/img/electrum_testnet-1.png)

Honestly, at first I was a bit lost with this one. I used the faucet address mentioned in the assignment, but for the life of me I could not find my *testnet address*.
I did what I usually do, start looking around and poking the program.

In the process managed to check that I was connected. 

![](/img/electrum_testnet-2.png)

I'm not sure ig this a bug with the Arch linux version (or maybe again problem with Wayland),
but the only way I could find my testnet address was by creating an empty receive request.  
Once created, my testnet address was displayed in the box on the right side. 

![](/img/electrum_testnet-4.png)

After that, I inserted my address to the faucets web page, did a CAPTCHA check and moments later, I was ~0.19 millibitcoins "richer".

| 1. Insert testnet address             | 2. Do CAPTCHA                         | 3. Profit..                                   |
| ------------------------------------- | -----------------------------------   | --------------------------------------------- |
| ![](/img/electrum_testnet-5.png)      | ![](/img/electrum_testnet-6.png)      | ![](/img/electrum_testnet-7.png)              |

## c

> c) Giveway. Move money to another Bitcoin wallet. Choose an amount where the last two digists are 73.

I hope I'm not on the wrong path with this assignment, but since it did not specify which or hows Bitcoin wallet to transfer funds, I created a new one.
The process was in my opinion pretty straight forward and I could find my way without difficulty.

| 1. Start transaction                      | 2. Confirm transaction                    | 3. review                                     |
| ----------------------------------------- | ----------------------------------------- | --------------------------------------------- |
| ![](/img/electrum-wallet-transfer-1.png)  | ![](/img/electrum-wallet-transfer-2.png)  | ![](/img/electrum-wallet-transfer-3.png)      |

I chose the *Static* method, because it seemed to be the cheapest one but the transaction remained *unconfirmed*. 
It seems that this is just related to the process on which transactions are processed, and it can take time. It seems that one can speed this up by increasing the transaction fee.
**Update**, the application is stating that the server is lagging, so maybe the only solutions is to wait.

## d

>d) Recycle. Move the testnet money back to the same faucet you got it from.

Since the previous transaction is still unconfirmed, I will return the balance I have left usable to the testnet faucet I got it from.

But for the life of me I could not find a refund function or the senders (input) address from the faucet transaction. From inputs the address was marked as *address unknown*...

So out of desperation, I used the the output address. I checked that it was not my own (from the transaction and the addresses tab) and send the reminder of my wealth to that address.

| Make payment                              | Review                                        |
| ----------------------------------------- | --------------------------------------------- |
| ![](/img/electrum-return-1.png)           | ![](/img/electrum-return-2.png)               |



## e

>e) Explorer. Use a block explorer to analyze a block on the real Bitcoin blockchain. Explain what each value and field means. You only need to analyze the block information and one sample transaction, as a block can contain many transactions. Voluntary bonus: Use a transaction that's interesting, such as one related to a crime or other unusual event.

For this I did a search with the web browser using keywords "*bitcoin explorer*", like the assignment suggested.
I settled on the [bitcoin explorer](https://bitcoinexplorer.org) website and selected the lates block 817177 (817177 is the height of the blockchain, meaning 817177 block after the genesis block(numbered 0)). Unfortunately I'm in a hurry, so I did not have time to search for a more interesting block.


| Main page                                 | Block 817,117                                 |
| ----------------------------------------- | --------------------------------------------- |
| ![](/img/explorer-1.png)                  | ![](/img/explorer-2.png)                      |

I'll try to analyze this block from the block summary JSON. I will list them if I can answer them.

{

    "hash": "00000000000000000001986fbb7c3a35efacc230e13177677079222715fb5594",
    "confirmations": 10,
    "height": 871177,
    "version": 939524096,
    "versionHex": "38000000",
    "merkleroot": "67841af82760bc246df5ea6de79a70dc471bb6f365f1baea6102d7db8033d936",
    "time": 1732101207,
    "mediantime": 1732098231,
    "nonce": 2286607580,
    "bits": "1702c070",
    "difficulty": 102289407543323.8,
    "chainwork": "00000000000000000000000000000000000000009b674ca0dda49fb8a4257b7c",
    "nTx": 5298,
    "previousblockhash": "00000000000000000000fefe5dd9a73ebe76efcb47a81d9b4cf4c4d54b3eba7c",
    "nextblockhash": "00000000000000000000f2cc52173ef74c6a758cff93f6e5d4a576da343c85ab",
    "strippedsize": 818074,
    "size": 1539156,
    "weight": 3993378,
    "tx": "See 'Transaction IDs'",
    "coinbaseTx": {
        "in_active_chain": true,
        "txid": "8e45bb7c46726a5d08304396ef50f4d2d77a35d8d54b540b33a98e6897b5c7ae",
        "hash": "3c29c25ce0e83ede01d631678fef266db88884dd6357bef6533850facbb0ac17",
        "version": 2,
        "size": 325,
        "vsize": 298,
        "weight": 1192,
        "locktime": 0,
        "vin": [
            {
                "coinbase": "03094b0d0457c43d672f466f756e6472792055534120506f6f6c202364726f70676f6c642f2e4ca15fd546030000000000",
                "txinwitness": [
                    "0000000000000000000000000000000000000000000000000000000000000000"
                ],
                "sequence": 4294967295
            }
        ],
        "vout": [
            {
                "value": 0.00000546,
                "n": 0,
                "scriptPubKey": {
                    "asm": "1 3daaca9b82a51aca960c1491588246029d7e0fc49e0abdbcc8fd17574be5c74b",
                    "desc": "rawtr(3daaca9b82a51aca960c1491588246029d7e0fc49e0abdbcc8fd17574be5c74b)#p35tdfgk",
                    "hex": "51203daaca9b82a51aca960c1491588246029d7e0fc49e0abdbcc8fd17574be5c74b",
                    "address": "bc1p8k4v4xuz55dv49svzjg43qjxq2whur7ync9tm0xgl5t4wjl9ca9snxgmlt",
                    "type": "witness_v1_taproot"
                }
            },
            {
                "value": 3.27624918,
                "n": 1,
                "scriptPubKey": {
                    "asm": "0 7086320071974eef5e72eaa01dd9096e10c0383483855ea6b344259c244f73c2",
                    "desc": "addr(bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38)#y9upg3rz",
                    "hex": "00207086320071974eef5e72eaa01dd9096e10c0383483855ea6b344259c244f73c2",
                    "address": "bc1qwzrryqr3ja8w7hnja2spmkgfdcgvqwp5swz4af4ngsjecfz0w0pqud7k38",
                    "type": "witness_v0_scripthash"
                }
            },
            {
                "value": 0,
                "n": 2,
                "scriptPubKey": {
                    "asm": "OP_RETURN aa21a9edd34b55d7990d724644bb6efa074146123a4f1b93c083909fac49e1df2e5b5740",
                    "desc": "raw(6a24aa21a9edd34b55d7990d724644bb6efa074146123a4f1b93c083909fac49e1df2e5b5740)#rk0550j3",
                    "hex": "6a24aa21a9edd34b55d7990d724644bb6efa074146123a4f1b93c083909fac49e1df2e5b5740",
                    "type": "nulldata"
                }
            },
            {
                "value": 0,
                "n": 3,
                "scriptPubKey": {
                    "asm": "OP_RETURN 434f5245012e50087fb834747606ed01ad67ad0f32129ab431e6d18fda214e5b9f350ffc7b6cf3058b9026e765",
                    "desc": "raw(6a2d434f5245012e50087fb834747606ed01ad67ad0f32129ab431e6d18fda214e5b9f350ffc7b6cf3058b9026e765)#eyjrnxlm",
                    "hex": "6a2d434f5245012e50087fb834747606ed01ad67ad0f32129ab431e6d18fda214e5b9f350ffc7b6cf3058b9026e765",
                    "type": "nulldata"
                }
            }
        ],
        "hex": "020000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff3103094b0d0457c43d672f466f756e6472792055534120506f6f6c202364726f70676f6c642f2e4ca15fd546030000000000ffffffff0422020000000000002251203daaca9b82a51aca960c1491588246029d7e0fc49e0abdbcc8fd17574be5c74bd6288713000000002200207086320071974eef5e72eaa01dd9096e10c0383483855ea6b344259c244f73c20000000000000000266a24aa21a9edd34b55d7990d724644bb6efa074146123a4f1b93c083909fac49e1df2e5b574000000000000000002f6a2d434f5245012e50087fb834747606ed01ad67ad0f32129ab431e6d18fda214e5b9f350ffc7b6cf3058b9026e7650120000000000000000000000000000000000000000000000000000000000000000000000000",
        "blockhash": "00000000000000000001986fbb7c3a35efacc230e13177677079222715fb5594",
        "confirmations": 10,
        "time": 1732101207,
        "blocktime": 1732101207
    },
    "totalFees": "0.15125464",
    "miner": {
        "name": "Foundry USA",
        "link": "https://foundrydigital.com/",
        "identifiedBy": "coinbase tag 'Foundry USA Pool'"
    },
    "subsidy": "3.125"

}

- "hash": "00000000000000000001986fbb7c3a35efacc230e13177677079222715fb5594"
  - The SHA256 hash of the block
- "confirmations": 10,
  - If I understand correctly, this tells you how many blocks have been generated after this one.
- "height": 871177
  - How many blocks ("steps") from the genesis block of the chain.
- "version": 939524096,
  - If I understand correctly, this is related to which block validation rules to follow (have no idea what these are).
- "merkleroot": "67841af82760bc246df5ea6de79a70dc471bb6f365f1baea6102d7db8033d936",
  - a hash of all the hashes of all the transactions in the block.
- "time": 1732102839,
  - Timestamp in UNIX time. When was the block created.
- "nonce": 2286607580,
  - An arbitrary number bitcoins miners change to modify the header hash to produce the target hash (right amount of leading zeros) for the block.
- "bits": "1702c070",
  - Difficulty. Determines the target hash size (amount of leading zeros) 
- "previousblockhash": "00000000000000000000fefe5dd9a73ebe76efcb47a81d9b4cf4c4d54b3eba7c",
  - a SHA256 hash of the previous block
- "nextblockhash": "00000000000000000000f2cc52173ef74c6a758cff93f6e5d4a576da343c85ab",
  - a SHA256 hash of the block generated after this one.
- "miner"
  - Who mined the block.

These are the most valid answers I could find, it seems that black headers are much smaller, consisting of:

- Hash
- Height
- Version
- Hash of previous the previous block header.
- Merkle root hash
- Timestamp
- Difficulty ("*Bits*")
- Nonce 

I picked a recent transaction on the block (made 20.11.2024 at 13:13). I*ll try to analyze it like the block previously.

{

    "in_active_chain": true,
    "txid": "c56b58c8d8d9dd20129322a8255b00c95506c3dad48720f7ad4cbbcb0f7f4002",
    "hash": "a1825a65032cbce3bf42746d379a8421c2e7c298694d4103bc57b5b5206042ca",
    "version": 2,
    "size": 210,
    "vsize": 128,
    "weight": 510,
    "locktime": 0,
    "vin": [
        {
            "txid": "fa6948d21a5b40a65cc26a7f34e99f9d8d23fbd3300f28e159d9cbe34b54e509",
            "vout": 1,
            "scriptSig": {
                "asm": "",
                "hex": ""
            },
            "txinwitness": [
                "3045022100f3d1f11459cec73fad62a9e94a148e2d85195095eab64d0583a26741b0550e1f02200fb166c84726390a88feaf29f9ed4ddd95c28d2c8f9fa8dab36afac90fb9fd8101",
                "032bfb22207e92351b0a2c17b91d0dbf2f0153d370d6663ee5d1baa23b2cbd63ee"
            ],
            "sequence": 4294967293
        }
    ],
    "vout": [
        {
            "value": 0,
            "n": 0,
            "scriptPubKey": {
                "asm": "OP_RETURN 13 148796351474",
                "desc": "raw(6a5d06148796351474)#lyeycg07",
                "hex": "6a5d06148796351474",
                "type": "nulldata"
            }
        },
        {
            "value": 0.00024368,
            "n": 1,
            "scriptPubKey": {
                "asm": "0 9557873a8c750ff1b739a112f2f4f910a285e3ee",
                "desc": "addr(bc1qj4tcww5vw58lrdee5yf09a8ezz3gtclwpjhtqf)#92yvgflj",
                "hex": "00149557873a8c750ff1b739a112f2f4f910a285e3ee",
                "address": "bc1qj4tcww5vw58lrdee5yf09a8ezz3gtclwpjhtqf",
                "type": "witness_v0_keyhash"
            }
        }
    ],
    "hex": "0200000000010109e5544be3cbd959e1280f30d3fb238d9d9fe9347f6ac25ca6405b1ad24869fa0100000000fdffffff020000000000000000096a5d06148796351474305f0000000000001600149557873a8c750ff1b739a112f2f4f910a285e3ee02483045022100f3d1f11459cec73fad62a9e94a148e2d85195095eab64d0583a26741b0550e1f02200fb166c84726390a88feaf29f9ed4ddd95c28d2c8f9fa8dab36afac90fb9fd810121032bfb22207e92351b0a2c17b91d0dbf2f0153d370d6663ee5d1baa23b2cbd63ee00000000",
    "blockhash": "00000000000000000001986fbb7c3a35efacc230e13177677079222715fb5594",
    "confirmations": 9,
    "time": 1732101207,
    "blocktime": 1732101207

}

- "in_active_chain": true,
  - Don't know, but if I had to guess, the bitcoin chain in which this transaction is stored is still active.
"txid": "c56b58c8d8d9dd20129322a8255b00c95506c3dad48720f7ad4cbbcb0f7f4002",
  - ID of the transaction, or specifically transaction hash.
- "value": 0.00024368,
  - Transaction value, in Bitcoins.
- "desc": "addr(bc1qj4tcww5vw58lrdee5yf09a8ezz3gtclwpjhtqf)#92yvgflj",
  - Senders address : bc1qj4tcww5vw58lrdee5yf09a8ezz3gtclwpjhtqf
- "address": "bc1qj4tcww5vw58lrdee5yf09a8ezz3gtclwpjhtqf",
  - Receivers address

Honestly this assignment was a bit rough. Finding answers for these took a while, since most searches lead to a more general definitions on bitcoin.

## f

>f) RogeCoin. Critically comment on [Honest Ads: If Cryptocurrency Was Honest](https://www.youtube.com/watch?v=GUs5y9leCyA) (Video, about 5 minutes). Identify and list arguments made. Provide commentary to support and challenge each of the claims. If you can, provide references or real life examples to your claims. (This task does not require tests with a computer.)

The video was highly entertaining and in my opinion brought up certain grievances I have regarding cryptocurrency in the modern world. First I thought that I would just make bullet point, but after pondering for a while, these turned out to be more of an essay.

## Crypto currency, a currency or an investment

The video highlights two factors:

1. You can't use crypto currency for buying goods and services
2. crypto currency extremely volatile
3. It's more suitable as an investment.

Firstly, the validity of a currency used in purchases is more often defined by the seller. Yes it's true that at the moment you can't by milk from you're local grocer using crypto currencies but that's the sellers choice. You could try and trade a carton of milk for one bitcoin (at todays exchange rate) and I think in this case a line would form up of willing sellers.
In a few years from know this could change all together. Society makes the decision what can and cannot be used for the exchange of goods and services and I think that crypto currencies in general are are little bit to new (even still) to gain wider acceptance.

This of course comes from the volatility of crypto currency value. Since crypto currencies lack a central bank that defines (or at least tries to influence it) the value, the value is defined by the users of it. This is a new setting and I'm not entirely sure what people make out of it. I'm leaning on the notion that people can't value these on the same basis that they value regular currency, since they are more interested on the monetary gain of the increased value of their investment, than it being suitable as a currency for general trading.  

I generally dislike the fact that crypto currencies are viewed as being more suitable as an investment, than being actually used in every day activity. If I have understood correctly, the need for crypto currencies came from the need to be free of governing monetary foundations and oversight. Money is power as they say and power corrupts. How I see this is that crypto currencies offer a shroud of anonymity for it's users, I know that maybe in Finland this is a nonissue, but what about people in countries where digital transactions are highly monitored and scrutinized on a state level? Money is a necessary (at least for the moment) evil in the world and controlling it is also a powerful tool. Of course this can make them attractive to criminal elements but at least crypto currency transactions are generally public. *Maybe I got a little carried away with this*.

Crypto currencies value in a investment form is volatile, because it's based on mutual understanding of that value and unfortunately in it's physical form is nothing. What defines the value of a startup? It's promise of success. I think that crypto currencies and their market value have gone haywire because of this promise, of something.

## Crypto currency as a status symbol

Yeah, maybe the image that the video refered to  not the right image 

## The need for technical knowledge of operation

I think this is a matter of opinion. Do most people know how modern money mechanics work with regular currencies?

## Computers solving needlessly complicated math problems

I would not called it needles, since the reason is to keep blockchain running and trustworthy.

## Consumption of energy and the destruction of the environment.  

I think this is also highly debatable, since crypto currency just need electricity, which can be produced with renewable methods. Also like the idea to use the excess heat to warm up homes and businesses. For example [google](https://energydigital.com/articles/how-googles-data-centre-waste-heat-is-heating-finnish-homes) is doing this in Hamina with their data centers.
I tried to find how much energy the "regular" monetary system uses on a daily bases for comparison but unfortunately I could not find any info on it.


## g

>g) Voluntary: Bib39ers. Write a bib39 phrase of a worthless wallet on a piece of paper. Hide it in your house to annoy thieves and others digging your stuff. Or leave it on a background of a photo and post it to social media. Optionally add or change a non-default word as an icing on a cake.

Unfortunately I ran out of time on this one..
