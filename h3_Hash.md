# H3 Hash

>*Welcome to the world of hashes!*
>
>*You'll use hashes for fingerprinting files, protecting passwords and as Bitcoin puzzles. You'll also take the role of the attacker, and crack some hashes with a well known tool, Hashcat.*

## Assignment

Full assignment can be viewed at the courses website : [h3 Hash](https://terokarvinen.com/trust-to-blockchain/#h3-hash)

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
>
>- € Schneier 2015: Applied Cryptography: Chapter 2 - Protocol Building Blocks: subchapters "2.3 One-way Fuctions" and "2.4 One-Way Hash Functions".
>- Karvinen 2022: Cracking Passwords with Hashcat
>- Voluntary bonus article (but recommended if you're not familiar with Linux command line): Karvinen 2020: Command Line Basics Revisited
>- Voluntary bonus article: € Santos et al 2017: Security Penetration Testing - The Art of Hacking Series LiveLessons: Lesson 6: Hacking User Credentials (8 videos, about 30 min)

### € Schneier 2015: Applied Cryptography: Chapter 2 - Protocol Building Blocks: subchapters "2.3 One-way Fuctions" and "2.4 One-Way Hash Functions".

The document summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Schneier 2015: Applied Cryptography](https://learning.oreilly.com/library/view/applied-cryptography-protocols/9781119096726/10_chap02.html#chap02-sec003)

The document details in general how one way and one-way hash functions work and are used in general and with some technical detail added but without going in to much to the computational details.

#### One-way functions

- One-way function in general is a function that is easy to compute, but very hard to reverse. Meaning that it is hard to deduce from the output that the function produces, what the original input was.
  - A trapdoor one-way function is an one-way function that works the same way but if you the secret to it (*The "trapdoor"*), you can easily compute the original input.
- The concept of one-way functions is pivotal in public-key cryptography.
- One-way functions are not the ideal solutions for encryption, since by their nature they are not meant to be decrypted.
  - A trapdoor one-way function would be a better choice for this, but like with public-key cryptography in general, keep the secret trapdoor a secret.

The document does state that there is no proof that one-way functions exist or can bo constructed if they are scrutinized mathematically, but I think this is relatable to the question *"What is actually random"*, so one would assume that they have to work one-way enough to work properly, in what I mean is that within the limits of our current technology, they should work as intended.

The document gave an example of a broken porcelain plate, which in my opinion was a good one, a plate easy to brake but hard to reassemble to it's original form.

#### One-way hash functions

- One-way hash functions can be considered one of the building blocks for modern cryptography and in one way or another have been used in computing for a long time. They work by digesting an input string (*String: a sequence of characters*), which is of variable length and converting it to a fixed length output string, or in other words, converting an arbitrary sized string to a fixed-sized string. The input is called a pre-image and the output a hash value.
- As with one-way functions, these functions work in general are one way, meaning that it's nearly impossible (Nearly, since in the long run with technology, nothing is impossible) to calculate the input from the output.
- Hash functions are made public, since the security of it lays in it's "*one-wayness*", so by examining it's code, one cannot form a method of reversing the output.
  - I would assume that there are private hahs functions in use, used in super secret applications but at the moment I can't confirm this.
- For a one-way hash function to work properly, it should be collision-free (Hash clash is a funny term I remember mentioned in my previous studies).
  - It should be hard to produce the same Hash value from different pre-images.
  - This makes each hash value a unique one, which is important in cryptography (Digital signatures for example) and comparing checksums (Verify integrity of a downloaded file for example). These two being only examples of many applications.

The document also explains the message authentication code (MAC), which is also called the data authentication code (DAC).It's a hash-function, that's hash-value can only be verified by a symmetric key used with the hash-function. It adds an extra layer of security, since only the ones with the symmetric key can verify the hash-value. This is not to be confused with Digital signatures, since only one key is used in this process.  I found this to be quite interesting, since the concept is new to me.

### Karvinen 2022: Cracking Passwords with Hashcat

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [Cracking Passwords with Hashcat](https://terokarvinen.com/2022/cracking-passwords-with-hashcat/)

### Karvinen 2020: Command Line Basics Revisited

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [Command Line Basics Revisited](https://terokarvinen.com/2020/command-line-basics-revisited/)

### € Santos et al 2017: Security Penetration Testing - The Art of Hacking Series LiveLessons: Lesson 6: Hacking User Credentials

The videos summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Security Penetration Testing The Art of Hacking Series LiveLessons](https://learning.oreilly.com/videos/security-penetration-testing/9780134833989/9780134833989-sptt_00_06_02_00/)


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
