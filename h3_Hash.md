# H3 Hash

>*Welcome to the world of hashes!*
>
>*You'll use hashes for fingerprinting files, protecting passwords and as Bitcoin puzzles. You'll also take the role of the attacker, and crack some hashes with a well known tool, Hashcat.*

## Assignment

Full assignment can be viewed at the courses website : [h3 Hash](https://terokarvinen.com/trust-to-blockchain/#h3-hash)

## Assignment answers

As with the previous assignment, all assignments in this report that require a computer and testing are performed by using my own PC.

### System environment

- Computer
  - CPU: AMD 7700 (Clocked @ 5.45 GHz)
  - Motherboard: ASUS B650M-plus-wifi
  - Memory: 32 GB DDR5 6000MHz
  - Graphics card: AMD RX 7900XTX
  - Storage: 1TB Western digital black SN850X (m.2, nvme)
  - Desktop: KDE plasma 6 (Wayland)
  - Package manager: pacman

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
- As with one-way functions, these functions work are one way, meaning that it's nearly impossible (Nearly, since in the long run with technology, nothing is impossible) to calculate the input from the output.
- Hash functions are made public, since the security of it lays in it's "*one-wayness*", so by examining it's code, one cannot form a method of reversing the output.
  - I would assume that there are private hahs functions in use, used in super secret applications but at the moment I can't confirm this.
- For a one-way hash function to work properly, it should be collision-free (Hash clash is a funny term I remember mentioned in my previous studies).
  - It should be hard to produce the same Hash value from different pre-images.
  - This makes each hash value a unique one, which is important in cryptography (Digital signatures for example) and comparing checksums (Verify integrity of a downloaded file for example). These two being only examples of many applications.

The document also explains the message authentication code (MAC), which is also called the data authentication code (DAC).It's a hash-function, that's hash-value can only be verified by a symmetric key used with the hash-function. It adds an extra layer of security, since only the ones with the symmetric key can verify the hash-value. This is not to be confused with Digital signatures, since only one key is used in this process.  I found this to be quite interesting, since the concept is new to me.

### Karvinen 2022: Cracking Passwords with Hashcat

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [Cracking Passwords with Hashcat](https://terokarvinen.com/2022/cracking-passwords-with-hashcat/)

In his article, Tero demonstrates how to use Hashcat fot password cracking. I like that he likes to remind the ethics and legal consideration of using penetration testing techniques, this is an important thing to remind now and again.

For this demonstration Tero uses *6b1628b016dff46e6fa35684be6acc96* as the hash to be cracked.

Tero uses Debian 11 Bullseye for this demonstration, running the linux os as a virtual machine in Virtual Box. He updates the system and installs hashcat and hashid. 
Hashid tries to identify what type of hash is used and hashcat is used for the cracking.

Tero downloads the rockyou.txt password dictionary, from Daniel Miessler's Github repository. The dictionary contains a list of passwords, leaked ones and ones that are some what obvious to guess. All in all the list contains over 14 million words.

Tero identifies the hash by using hashid and it identifies it to be MD2, MD4 or MD5. Tero determines that it could be MDS5, since it's more common than MD2 or MD4.
He uses hashcat to crack the the hash and it turns out that MD5 was the correct assumption and the hash is revealed to be *summer*. The article does not specify this, but hascat goes through the rockyou.txt dictionary, generating a MD5 hash of the words and comparing it with the hash attempted to be cracked. As explained in the article, if a match with the hashes is found, it reports the correct hash and the word that produces it. 
The cracking was relatively fast, taking 4.6 milliseconds. The cracking speed was 37401.1 kH/s, 37 million words a second. Hard to say if this is fast or not, but Tero also suggest that using real hardware (not a virtual machine) and a GPU will make this even faster. I think I should give my AMD card a try, Tero mentions Nvidia but it has it's problems with open source. [Linus Torvalds also had an opinion on this](https://www.youtube.com/watch?v=_36yNWw_07g)

I like Tero's articles. Having learned from them in the past, it's nice to return to them in this course.

### Karvinen 2020: Command Line Basics Revisited

The document summarized can be read at the Tero Karvinen website, **Note that the site  does not require registration**  : [Command Line Basics Revisited](https://terokarvinen.com/2020/command-line-basics-revisited/)

### € Santos et al 2017: Security Penetration Testing - The Art of Hacking Series LiveLessons: Lesson 6: Hacking User Credentials

The videos summarized can be read at the O'Reilly website, **Note that the site requires registration**  : [Security Penetration Testing The Art of Hacking Series LiveLessons](https://learning.oreilly.com/videos/security-penetration-testing/9780134833989/9780134833989-sptt_00_06_02_00/)


## a

>a) Billion dollar busywork. Command 'echo -n "hello"|sha256sum' prints a hash. Try adding something to the string, e.g. 'echo -n 'hello asdf'|sha256sum'. What do you have to add to get a hash that starts with a zero? (Voluntary bonus: How is this related to Bitcoin? Voluntary difficult bonus: How many zeros can you get to the beginning? Voluntary difficult bonus: How does the difficulty raise?)

It took me a few tries to get the leading zero, in this case the string that produced is was *hello-adsf13*

![](/img/generating_hashes_with_zeros-1.png)

>Voluntary bonus: How is this related to Bitcoin?

If i'm not completely wrong, the leading zeros in a SHA256 hash is related to the Bitcoin mining difficulty level. The bitcoin network has a set block difficulty called target (a 256-bit number) and when mining a new block, the SHA256 hash of the blocks header must be lower than the set target. So if the target is *n* computationally you should guess a string that produces a SHA256 hash with *n* leading zeros. It's described as being more of a lottery than a long and determined process of finding the correct string.

**Sources:**
- [What is bitcoin mining difficulty](https://www.bitcoinmining.com/what-is-bitcoin-mining-difficulty/)
- [Bitcoin mining difficulty](https://en.bitcoin.it/wiki/Difficulty/)
- [Target](https://en.bitcoin.it/wiki/Target)

## b

>b) Compare hash. Create a small text file. Take it's hash (e.g. 'sha256sum tero.txt'). Change one letter. Take the hash again. Compare hashes. What do you notice?

I created a text file and added word *testhash* to it, generated a hash using SHA256, changed one letter so the word is now *testcash* and generated a new hash.
Comparing the hashes we find out that thet are completely different.

![](/img/testing_hash-1.png)

## c

> c) Hashcat. Install hashcat and test that it works.

With Arch linux it seems that Hashcat is found in the extra repositories but hashid is available only from AUR. So we have to use two package managers, *pacman* and *yay*. So I started by installing both packages.

![](/img/hashcat-1.png)

*Note that I'm working from this projects Git repository, so it gives a nice touch on the terminal*

For testing I'm using the same rockyou.txt dictionary that Tero used in his demonstration used in subtask [Answer to assignment x](h3_Hash.md#x).
Lets download if from Daniel Miessler's Github repository, I created a new folder "*testing*" and downloaded the dictionary there. **NOTE** that the "*testing*" folder is marked in the .gitignore file and thus is not included in this repository.

![](/img/hashcat-2.png)

Once we have downloaded, extracted and checked the rockyou.txt file we can test the new programs. Since this is only a test, let's use the same hash as Tero did in his previous demonstration. This being *6b1628b016dff46e6fa35684be6acc96*.

### Problems..

While testing I found out that hashcat was quite a picky on drivers. It did not work with my AMD CPU and it took me over an hour to get it to it to work with an AMD GPU (discrete or integrated).

In short:
  - With Arch linux, use the AUR package for hascat (the arch linux hascat package did not work correctly).
    - Install [hashcat-git](https://aur.archlinux.org/packages/hashcat-git)
  - For AMD GPU, install tne rocm opencl package [rocm-opencl-runtime](https://archlinux.org/packages/?name=rocm-opencl-runtime)
    - Tried the opencl-amd and the hip amd drivers with out success.

And again, I found the answer from [Arch wiki](https://wiki.archlinux.org/title/GPGPU) :)

### On with the testing

Now that we have confirmed that hascat works, let's do a proper test with it and document it. This is a pretty much recreates the demonstration in Tero's article : [cracking-passwords-with-hashcat](https://terokarvinen.com/2022/cracking-passwords-with-hashcat/).

It turns out that hashcat selected the integrated GPU by default. Leaving the cracking speed somewhat lacking (10342.7 KH/s), by selecting the correct device *-d 1*, I was able to get a "bit" more speed to it, raising the cracking speed to 316.0 MH/s. Funny enough, also adding the the device parameter cleared an reported error in processing. 

| Running with defaults                     | Selecting the discrete GPU                    |
| ----------------------------------------- | --------------------------------------------- |
| ![](/img/hashcat-3.png)                   | ![](/img/hashcat-4.png)                       |

The hash is cracked and the result is the same as in Tero's demonstration: summer.

**NOTE:** Hashcat reports that:

*ATTENTION! Pure (unoptimized) backend kernels selected. Pure kernels can crack longer passwords, but drastically reduce performance. If you want to switch to optimized kernels, append -O to your commandline. See the above message to find out about the exact limits.*

But if I the optimized kernel (parameter *-O*), the maximum password length supported by the kernel drops from 256 to 31. This could be OK, but we never know if someone is using a password thats longer than 31 characters.

### Summary

Even though this part was quite frustrating with the problems, I got to learn something new in using hashcat.

## d

>d) Dictionary attack. Crack this hash: 21232f297a57a5a743894a0e4a801fc3

I will use same same dictionary as before (rockyou.txt) since it's nicely available. We start off by identifying what has is in question and then try cracking it.

Tero suggested that when using hashid, it's likely that one of the first three results is most likely right. The first three are MD2, MD4 and MD5, I will try MD5, since thats more common than MD2 or MD4.

![](/img/hashcat-5.png)

It turns out that the result is *"admin"*. (Summer is the result from out last cracking)

## e

>e) How can you make a password that's protected against a dictionary attack?

Since dictionary attacks are based on known, or guessed passwords that have been added to the dictionary (or dictionaries) used in the attack, using a randomly generated password, like *L25:3n}R]P?<"=gt~,rzpy* protects against this. I would say add the word "should protect" since one could try and computationally guess the right randomly generated password but this could take a ridiculously long time (hundreds or thousands or millions of years), so in practice using long (24 characters or more) randomly generated passwords adds protection against dictionary attacks. It should be noted that dictionary attacks are often used for username and password combinations, so using a not so obvious username also adds a layer of protection. This of course is meaningless if the username is already known to the attacker. 

## f

>f) Voluntary: Two minute job. Try cracking this hash and comment on your hash rate $2y$18$axMtQ4N8j/NQVItQJed9uORfsUK667RAWfycwFMtDBD6zAo1Se2eu . This subtask d does not require actually cracking the hash, just trying it and commenting on the hash rate.

This actually was a two minute assignment. When I started cracking this hashcat warned that *performance is lower than expected* and it truly was, with a MD5 hash cracking speed was 316.0 MH/s, with Blowfish it was just 13 H/s. So I started the cracking and went to lunch, when I came back hascat was finished and it had taken it just a little over two minutes. The hash turned out to be 12345. But in reality this hash rate is really. really slow.

![](/img/blowfish-1.png)

## g

>g) Voluntary bonus: Where do you want to go today? Crack this Windows NTLM hash: f2477a144dff4f216ab81f2ac3e3207d

Hashid indetified the top three has types as MD2, MD5 and MD4. Without the info of NTML, this could have taken much longer, since the hash type would be based on guessing.

![](/img/NTLM_hash_1.png)

The cracked hash turned output to be *monkey*, so I guess I'm going to the zoo.

## h

>h) Voluntary bonus: Embarassingly parallel. Make hashcat work with your display adapter (GPU). Compare hash rate with and without GPU enabled.

I kind of answered this in [assignment c](h3_Hash.md#c) while installing hashcat and dealing with problems regarding it's use.
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
