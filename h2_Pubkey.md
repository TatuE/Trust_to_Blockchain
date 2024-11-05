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

The document starts by describing the principals on how public-key cryptography works and explains it's history. The concept of public-key cryptography was presented by Whitfield Diffie and Martin Hellman in 1976, although NSA claims that they already new of the concept in 1966. The proposed model used cryptographically generated key pairs, one public key for encrypting a message and one private key for decrypting the message. The model is based on a mathematical trap-door one-way functions which makes the encryption easy, but decryption extremely hard.

The public-key cryptography intends to answer the problems regarding symmetric key cryptography (symmetric-key algorithm), in which the encryption is based on a single, shared key and is there for based around keeping that key a secret between the parties using it. The public key, as the name states, can be made available publicly with out outright endangering the message confidentiality. The private key on the other hand is the weak link of this process and should be, as the name states, kept private. In practical application public-key cryptography is not intended to replace symmetric key cryptography, since public-key algorithms are much slower than symmetric-key algorithms but to work along side it by encrypting a session key (symmetric) to be used in communication, this is called a hybrid cryptosystem.

##### Hybrid cryptosystem in short

Alice and Bob a normally used as names for the participants in an example, but to keep thing interesting, I will use Pinky an the Brain (Brain for short).

1. The Brain sends his public key to Pinky.
2. Pinky generates a session key using, let's say using AES-128 (Advanced Encryption Standard).
3. Pinky encrypts the session key using the Brains public key.
4. Pinky sends the encrypted session key to the Brain.
5. The Brain decrypts Pinky's message using his private key.
6. Pinky and the Brain encrypt their communication using the aforementioned session key and try to form a plan to take over the world.

Public-key cryptography can be applied to produce digital signatures to proof authorship and identity, replacing hand written signatures.
Digital signatures can be produced and verified using symmetric cryptography and an arbitrator in the middle, but this system can be time consuming and relies heavily on the trust of the arbitrator. 

In some public-key algorithms, like RSA, both keys can be used for encryption. In this scenario encrypting a message with a private key offers the proof of ownership and identity, assuming the private key has not been compromised. Using this method the message encrypted with the private key can be decrypted with the public key and vice versa. The key pairs are generated in a way (The public key is generated from the private key), that the private key cannot be deduced from the public key.

To make the digital signatures produced by using public-key algorithms in documents more secure and practical, timestamps and one-way hash functions (like SHA [Secure Hash Algorithm]) should be used. 

- Timestamps prevents the reuse of a signature, since it includes the time and date when the signature is created. This of course assumes that this is logged.
- One-way hash functions provide proof that the signature is related to the signed document and thus valid.
    - Saving the signed hash as proof is also a way to save storage space, since its smaller that the original document. 

Since the encrypted hash (in this case the timestamped digital signature) is related to a single person or organization, signatures by multiple parties can be applied to a single document.

As mentioned before, since public-key cryptography relies on the assumption that the private key is kept secret, this provides a possible weak point with digital signatures. A signing member can afterwards state that their private key has been compromised and they in fact did not sign the document. This can be be remedied by verification, the signed messages are timestamped on reception and returned as a confirmation receipt to the sender.

Using one-way hash functions also brings an additional benefit in digital signatures, it can proof that the signed document (or message) is unaltered. The recipient can produce a hash from the document using the same hash function and compare it to the decrypted hash from the sender. 

Using encryption in sending the encrypted hash (digital signature) provides even more security and reassurance of the senders identity. In this scenario you would sign the hash with you're private key and encrypt a message containing said encrypted has with the recipients public key. 

Public-key cryptography relies on trust and this is also a weak point in the system. How to know that a persons public key is in fact they're public key if it is made publicly available? One way is to use a Key Certification Authority or Key Distribution Center, who can help verify the authenticity of the said key.

The document also details how random is randomly generated numbers, this is important since cryptographic keys are based on randomly generated numbers.
The conclusion is that computationally cannot be truly (strictly speaking) random, since if huge measures are taken to try an reproduce the conditions in which the number is generated, it might produce the same results but in real life scenarios it only needs to be random enough to work, meaning that it cannot be reliably reproduced.

### Rosenbaum 2019: Grokking Bitcoin: Chapter 2. Cryptographic hash functions and digital signatures, sections "Typical use of digital signatures" to "Private key security".

The document summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Rosenbaum 2019: Grokking Bitcoin](https://learning.oreilly.com/library/view/grokking-bitcoin/9781617294648/OEBPS/Text/kindle_split_011.html#ch02lev2sec8)

The document demonstrates in more details the usage of digital signatures mentioned in the previous summary.

It points out that while hand written signatures are usually tied to the person writing the signature, digital signatures are tied to the private key that signs them.




### Karvinen 2023: PGP - Send Encrypted and Signed Message - gpg

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [PGP - Send Encrypted and Signed Message - gpg](https://terokarvinen.com/2023/pgp-encrypt-sign-verify/)

In his article, Tero demonstrates how to send an encrypted and signed message using openPGP encryption. The encryption and decryption is done using GPG (GNU Privacy Guard)

- Tero starts by generating a key-pair using GPG, which produces private (secret) and a public key (derived from the private key). The private key can be secured using a password, but in this demonstration password protection is not applied. Once created, the public key is exported, so it can be send to a recipient (Alice in this demonstration). 
- Tero simulates an exchange of public key between him and Alice. Alice in this case is a folder and a separate key-pair is generated for Alice. In both cases Tero lists the fingerprints of the keys generated.
- Tero and Alice exchange they're public keys and in this case the fingerprint verification is done by a simulated phone call between Tero and Alice. Once the fingerprint of the public key is verified, it can be signed as a trusted key.
- Alice creates a message, signs it with her private key and encrypts is with Tero's public key. Tero points out that after this, Alice cannot decrypt the message, since only Tero's private key can decrypt it. The message is send (moved to Tero's folder) for Tero to decrypt.
- Tero decrypts the message received from Alice and verifies he signature, that he has verified as trusted.

## a

>a) Pubkey today. Explain how you have used public key cryptography today or yesterday, outside of this homework. In addition to naming the system, identify how different parties use keys in different steps of the system. (Answering this question likely requires finding sources on your own. This subtask does not require tests with a computer.)

### TLS

Source: [TLS handshake](https://www.cloudflare.com/learning/ssl/what-happens-in-a-tls-handshake/)

When you browse the internet and use HTTPS, the connection is secured using a symmetric key but that session key and the selected cipher suite is negotiated by a TLS (transport layer security) handshake that uses public key cryptography. The process is as follows:

1. The client connects to the HTTPS server and requests a secured connection. The client message specifies what TLS version (1.3 is the newest) and cipher suite it supports and a random string of bytes generated by the client. 
2. The HTTPS server sends the client a message containing the servers SSL certificate (public key), the selected cipher suite and a random string of bytes generated by the server.
3. The client verifies the SSL certificate with the certificate authority (CA) that has provided it.
4. If the SSL certificate is confirmed by the CA, the process continues, otherwise the secure connection attempt stops.
5. When the certificate is confirmed, the client generates a another random string of bytes. This is called the **premaster secret**. The premaster secret is encrypted by the HTTPS servers public key (SSL certificate) and send to the HTTPS server.
6. The HTTPS server decrypts the message using it's private key. 
7. Both the client and server generate a session key (symmetric cryptography) based on the random string of bytes sent (client, server and premaster secret). This is based on the cipher suite selected and both should produce the same session key.
8. The client and server finnish the handshake by using encrypted messages encrypted with the session key.
9. Secure communication continues using the session key.

There are few definitions may not be that self evident, these are certificate authority (CA) and cipher suite.

#### Certificate authority (CA)

Source: [certificate authority (CA)](https://www.techtarget.com/searchsecurity/definition/certificate-authority)

A certificate authority is an entity, that provides SSL certificates to be used by a domain. The role of the CA is based on mutual trust, so when the client verifies a certificate (public key) with the CA, it can confirm that the certificate was issued by the said authority and that the SSL certificate holder is truly who they claim to be (a domain, like example.com for instance). 


#### Cipher suite

The cipher suite determines what set of algorithms (RSA, AES, SHA for example) are used in a secured communication session.

## b

>b) Messaging. Send an encrypted and signed message using PGP, then verify and decrypt it. (You can use folders to simulate users, or use two computers or two different OS users. Don't use Tero as a name of any party, unless that's your given name.)

### Used OS and other considerations

- As with the previous assignment, this assignment is done in EndeavourOS (Arch linux).
- For practical reasons, use SSH for client connection if doing this with multiple users in the same OS, at the same time. Pinetry (used by gnupg) fails otherwise and you cannot create keys

### The assignment

For this assignment I created two new users in my Linux system, Pinky and Brain. For the sake of convenience I created a new group called "mailBoxUsers" and added Pinky and Brain to this group. For messaging, I created a folder named mailBox in home folder, with mail boxes for Pinky and Brain so they can leave messages for each other. The access rights for the mailBox folder is set to the root and the group "mailBoxUsers", so Pinky and Brain have the right to leave messages for one another.

![](/img/GPG_example-1.png)

 **NOTE that the folder rights should be modified so that all new files are assigned to the correct group**
 - *chmod -R g+s mailBox*

Before we start, we have to install gnupg. This is done by the system administrator (me) of the system who has sudo rights.

- *sudo pacman -S gnupg*

 We will start by generating a key pair for Pinky and Brain. We will use the defaults and not using password protection but know that **This is advisable in real life!!**

![](/img/GPG_example-2.png)

 Next we export the public keys and store them in the receivers mailBox folders.

![](/img/GPG_example-3.png)

The files only have read rights for the group, but this should be OK. 

Next we import the keys and verify them.

Pinky and Brain hold a huge amount of trust with one another, so no need to verify the origins. They could of course check the the files owners and determine the keys trustworthiness that way.

![](/img/GPG_example-4.png)

Next Pinky will send a secret message to the Brain.
The message is written using Vi (not Emacs). Ok, it's Vim but you get the idea. 

![](/img/GPG_example-5.png)

Pinky will not sing the the message and encrypt it with the Brains public key. 

![](/img/GPG_example-6.png)

Pinky sends (moves) the encrypted message to Brains mailBox and also modifies the access rights so that the group has write rights to it. He forgot to do this before sending it.

![](/img/GPG_example-7.png)

Brain is delighted to receive a message in his mailbox and decrypts it.

![](/img/GPG_example-8.png)

The message is from Pinky, he writes a response for Pinky. Brain encrypts the message same way as Pinky did, assigns the same access rights to it and moves it to Pinky's mailbox.

![](/img/GPG_example-9.png)

Pinky receives the message and decrypts it to find the answer for tonights plans.

![](/img/GPG_example-10.png)

Pinky is delighted that tonights plans are the same as always.

## c

>c) Other tool. Encrypt a message using a tool other than PGP. Explain how different parties use different keys at different stages of operation. Evaluate the security of the tool you've chosen.

## d

>d) Eve and Mallory. In many crypto stories, Eve is a passive eavesdropper, listening on the wire. Mallory maliciously modifies the messages. Explain how PGP protects against Mallory and Eve. Be specific what features, which use of keys and which flags in the command are related to this protection. (This subtasks does not require tests with a computer)



## f

>f) Password management. Demonstrate use of a password manager. What kind of attacks take advantage of people not using password managers? (You can use any password manager, some examples include [pass](https://www.passwordstore.org/) and [KeePassXC](https://keepassxc.org/).)

I have been using Pass since 2017 (at least), it started out as my primary password manager that I self hosted, but since last summer, it has become more of a super secret repository (server passwords, social security numbers) that is accessible only from my local network. For more daily passwords (like Haaga-Helia) I use Pass provided by Proton AG, since it's more easily accessible. I'm planning on hosting Vaultwarden (An alternative server implementation of the Bitwarden Client API, written in Rust) 

## g

>g) Refer to sources. Verify each homework report (this and the earlier ones) refers to sources. Every homework report should refer to this task page. It should also have references to any other source used, such as web pages, LLMs, man pages, other reports... References are mandatory, and must be present in every report. (This subtask does not need a report, you can just do it and write "Done." as the answer for this subtask.)

Done

## h

>h) Voluntary, challenging, requires coding: Cryptopals: Challenge Set 1:
>1. Convert hex to base64 (feel free to use a library for base64)
>2. Fixed XOR
>3. Single-byte XOR cipher
>4. Detect single-character XOR (This looks tough before you have solved 1-3)