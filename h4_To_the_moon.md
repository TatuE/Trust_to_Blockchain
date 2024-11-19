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

## d

>d) Recycle. Move the testnet money back to the same faucet you got it from.

## e

>e) Explorer. Use a block explorer to analyze a block on the real Bitcoin blockchain. Explain what each value and field means. You only need to analyze the block information and one sample transaction, as a block can contain many transactions. Voluntary bonus: Use a transaction that's interesting, such as one related to a crime or other unusual event.

## f

>f) RogeCoin. Critically comment on [Honest Ads: If Cryptocurrency Was Honest](https://www.youtube.com/watch?v=GUs5y9leCyA) (Video, about 5 minutes). Identify and list arguments made. Provide commentary to support and challenge each of the claims. If you can, provide references or real life examples to your claims. (This task does not require tests with a computer.)

## g

>g) Voluntary: Bib39ers. Write a bib39 phrase of a worthless wallet on a piece of paper. Hide it in your house to annoy thieves and others digging your stuff. Or leave it on a background of a photo and post it to social media. Optionally add or change a non-default word as an icing on a cake.
