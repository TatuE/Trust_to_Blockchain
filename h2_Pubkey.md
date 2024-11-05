# H2 Pubkey

>*You're using public key encryption every day. If you know what you're doing, it can help you even more.*

## Assignment

Full assignment can be viewed at the courses website : [h2 Pubkey](https://terokarvinen.com/trust-to-blockchain/#h2-pubkey)

## Assignment answers

### Assignment links

Links to assignment answers.

- [Answer to assignment x](h2_Pubkey.md#x)
- [Answer to assignment a](h2_Pubkey.md#a)
- [Answer to assignment b](h2_Pubkey.md#b)
- [Answer to assignment c](h2_Pubkey.md#c)
- [Answer to assignment d](h2_Pubkey.md#d)
- [Answer to assignment f](h2_Pubkey.md#f)
- [Answer to assignment g](h2_Pubkey.md#g)
- [Answer to assignment h](h2_Pubkey.md#h)

## x

>x) Read and summarize (with some bullet points)
>- € Schneier 2015: Applied Cryptography: Chapter 2 - Protocol Building Blocks, sections
>    - 2.5 Communications Using Public-Key Cryptography
>    - 2.6 Digital Signatures
>    - 2.7 Digital Signatures With Encryption
>    - 2.8 Random And Pseudo-Random-Sequence Generation
>- € Rosenbaum 2019: Grokking Bitcoin:
>    - Chapter 2. Cryptographic hash functions and digital signatures:
>        - Digital signatures (8 sections, from "Typical use of digital signatures" to "Private key security")
>- Karvinen 2023: PGP - Send Encrypted and Signed Message - gpg

### Schneier 2015: Applied Cryptography: Chapter 2 - Protocol Building Blocks, sections 2.5 - 2.8

The document summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Schneier 2015: Applied Cryptography](https://learning.oreilly.com/library/view/applied-cryptography-protocols/9781119096726/10_chap02.html#chap02-sec005)

Chapter 2.5 describes the principals on how public-key cryptography works and explains it's history. The concept of public-key cryptography was presented by Whitfield Diffie and Martin Hellman in 1976, although NSA claims that they already new of the concept in 1966. The proposed model used cryptographically generated key pairs, one private and one public. The private key could decrypt a message encrypted by the public key.


### Rosenbaum 2019: Grokking Bitcoin: Chapter 2. Cryptographic hash functions and digital signatures, sections "Typical use of digital signatures" to "Private key security".

The document summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Rosenbaum 2019: Grokking Bitcoin](https://learning.oreilly.com/library/view/grokking-bitcoin/9781617294648/OEBPS/Text/kindle_split_011.html#ch02lev2sec8)


### Karvinen 2023: PGP - Send Encrypted and Signed Message - gpg

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [PGP - Send Encrypted and Signed Message - gpg](https://terokarvinen.com/2023/pgp-encrypt-sign-verify/)

In his article, Tero demonstrates how to send an encrypted and signed message using openPGP encryption. The encryption and decryption is done using GPG (GNU Privacy Guard)



## a

>a) Pubkey today. Explain how you have used public key cryptography today or yesterday, outside of this homework. In addition to naming the system, identify how different parties use keys in different steps of the system. (Answering this question likely requires finding sources on your own. This subtask does not require tests with a computer.)

### TLS

https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/

When you browse the internet and use HTTPS, the connection is (often) secured using public key cryptography. **MORE TEXT**

### SSH

https://www.ssh.com/academy/ssh/public-key-authentication

SSH (Secure SHell) can use public key cryptography for authentication.

### Linux software repositories

https://www.redhat.com/en/blog/rpm-gpg-verify-packages


## b

>b) Messaging. Send an encrypted and signed message using PGP, then verify and decrypt it. (You can use folders to simulate users, or use two computers or two different OS users. Don't use Tero as a name of any party, unless that's your given name.)

## c

>c) Other tool. Encrypt a message using a tool other than PGP. Explain how different parties use different keys at different stages of operation. Evaluate the security of the tool you've chosen.

## d

>d) Eve and Mallory. In many crypto stories, Eve is a passive eavesdropper, listening on the wire. Mallory maliciously modifies the messages. Explain how PGP protects against Mallory and Eve. Be specific what features, which use of keys and which flags in the command are related to this protection. (This subtasks does not require tests with a computer)

## f

>f) Password management. Demonstrate use of a password manager. What kind of attacks take advantage of people not using password managers? (You can use any password manager, some examples include [pass](https://www.passwordstore.org/) and [KeePassXC](https://keepassxc.org/).)

I have been using Pass since 2017 (at least), it started out as my primary password manager that I self hosted, but since last summer, it h 

## g

>g) Refer to sources. Verify each homework report (this and the earlier ones) refers to sources. Every homework report should refer to this task page. It should also have references to any other source used, such as web pages, LLMs, man pages, other reports... References are mandatory, and must be present in every report. (This subtask does not need a report, you can just do it and write "Done." as the answer for this subtask.)

## h

>h) Voluntary, challenging, requires coding: Cryptopals: Challenge Set 1:
>1. Convert hex to base64 (feel free to use a library for base64)
>2. Fixed XOR
>3. Single-byte XOR cipher
>4. Detect single-character XOR (This looks tough before you have solved 1-3)