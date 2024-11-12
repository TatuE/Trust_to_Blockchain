# H3 Hash

>*Welcome to the world of hashes!*
>
>*You'll use hashes for fingerprinting files, protecting passwords and as Bitcoin puzzles. You'll also take the role of the attacker, and crack some hashes with a well known tool, Hashcat.*

## Assignment

Full assignment can be viewed at the courses website : [h2 Pubkey](https://terokarvinen.com/trust-to-blockchain/#h3-hash)

## Assignment answers

### Assignment links

Links to assignment answers.

- [Answer to assignment x](h3_Hash.md#x)
- [Answer to assignment a](h3_Hash.md#a)
- [Answer to assignment b](h3_Hash.md#b)
- [Answer to assignment c](h3_Hash.md#c)
- [Answer to assignment d](h3_Hash.md#d)
- [Answer to assignment e](h3_Hash.md#e)
- [Answer to assignment f](h3_Hash.md#f)
- [Answer to assignment g](h3_Hash.md#g)
- [Answer to assignment h](h3_Hash.md#h)
- [Answer to assignment i](h3_Hash.md#i)
- [Answer to assignment j](h3_Hash.md#j)
- [Answer to assignment k](h3_Hash.md#k)
- [Answer to assignment l](h3_Hash.md#l)

## x

>x) Read and summarize (with some bullet points)
>- € Schneier 2015: Applied Cryptography: Chapter 2 - Protocol Building Blocks: subchapters "2.3 One-way Fuctions" and "2.4 One-Way Hash Functions".
>- Karvinen 2022: Cracking Passwords with Hashcat
>- Voluntary bonus article (but recommended if you're not familiar with Linux command line): Karvinen 2020: Command Line Basics Revisited
>- Voluntary bonus article: € Santos et al 2017: Security Penetration Testing - The Art of Hacking Series LiveLessons: Lesson 6: Hacking User Credentials (8 videos, about 30 min)

## a

>a) Billion dollar busywork. Command 'echo -n "hello"|sha256sum' prints a hash. Try adding something to the string, e.g. 'echo -n 'hello asdf'|sha256sum'. What do you have to add to get a hash that starts with a zero? (Voluntary bonus: How is this related to Bitcoin? Voluntary difficult bonus: How many zeros can you get to the beginning? Voluntary difficult bonus: How does the difficulty raise?)

## b

>b) Compare hash. Create a small text file. Take it's hash (e.g. 'sha256sum tero.txt'). Change one letter. Take the hash again. Compare hashes. What do you notice?

## c

> c) Hashcat. Install hashcat and test that it works.

## d

>d) Dictionary attack. Crack this hash: 21232f297a57a5a743894a0e4a801fc3

## e

>e) How can you make a password that's protected against a dictionary attack?

## f

>f) Voluntary: Two minute job. Try cracking this hash and comment on your hash rate $2y$18$axMtQ4N8j/NQVItQJed9uORfsUK667RAWfycwFMtDBD6zAo1Se2eu . This subtask d does not require actually cracking the hash, just trying it and commenting on the hash rate.

## g

>g) Voluntary bonus: Where do you want to go today? Crack this Windows NTLM hash: f2477a144dff4f216ab81f2ac3e3207d

## h

>h) Voluntary bonus: Embarassingly parallel. Make hashcat work with your display adapter (GPU). Compare hash rate with and without GPU enabled.

## i

>i) Voluntary bonus: My hash. create some hashes of your own, then crack them with hashcat.

## j

>j) John. Install Jumbo John (John the Ripper, open source Jumbo version). Compile it from source code as needed. See Karvinen 2023 Crack File Password With John.

## k

>k) Crack file password with John.

## l

>l) Voluntary bonus: Custom dictionary. What's Elmeri's password?
>
>Loot from Elmeri's machine
>
>*$ sudo grep elmik9 /etc/passwd /etc/shadow*  
>*/etc/passwd:elmik9:x:1003:1003:Elmeri "9" Elmik,,,:/home/elmik9:/bin/bash*  
>*/etc/shadow:elmik9:$1$xpRkwrhq$aXdu7HQirUmuTZW2m8OXs.:18401:0:99999:7:::*  
