# h6 Upside Down Iceberg

>*In Finland, it's legal to use TOR at the time of writing. If you reside in another juristiction, laws might be different. Obviously, it's illegal to do illegal things in TOR, just like it's illegal to do illegal things anywhere. Only do legal things.*
>
>*If you reside in a jurisdiction where using TOR is illegal, you obviously can't install it and do the related tasks. If you cannot or do not want to do the hands-on darknet tasks, the alternative task is: based on literature only (no hands on tests, no installation), compare anonymous/pseudonymous networks, such as TOR, I2P, Freenet and others. How do their goals, technology and other features differ? How are they similar? Add references. Link differences and benefits to technical and architecture aspects.*

## Assignment

Full assignment can be viewed at the courses website : [h6 Upside Down Iceberg](https://terokarvinen.com/trust-to-blockchain/#h6-upside-down-iceberg)

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

- [Answer to assignment x](h6_Upside_Down_Iceberg.md#x)
- [Answer to assignment a](h6_Upside_Down_Iceberg.md#a)
- [Answer to assignment b](h6_Upside_Down_Iceberg.md#b)
- [Answer to assignment c](h6_Upside_Down_Iceberg.md#c)
- [Answer to assignment d](h6_Upside_Down_Iceberg.md#d)
- [Answer to assignment e](h6_Upside_Down_Iceberg.md#e)
- [Answer to assignment f](h6_Upside_Down_Iceberg.md#f)
- [Answer to assignment g](h6_Upside_Down_Iceberg.md#g)
- [Answer to assignment h](h6_Upside_Down_Iceberg.md#g)
- [Answer to assignment i](h6_Upside_Down_Iceberg.md#g)
- [Answer to assignment j](h6_Upside_Down_Iceberg.md#g)

## x

>- x) Read and summarize (briefly, e.g. with some bullets)
>  - Dingledine, Mathewson and Syverson 2004: [Tor: The second-generation onion router.](https://css.csail.mit.edu/6.858/2022/readings/tor-design.pdf) In USENIX security symposium ([jufo](https://jfp.csc.fi/jufoportaali) level 2). Chapter:
>    - 3 Design goals and assumptions
>  - Karunanayake, Ahmed, Malaney, Islam and Jha 2021: [De-anonymisation attacks on tor: A survey](https://ieeexplore.ieee.org/ielx7/9739/9621320/09471821.pdf). In IEEE Communications Surveys & Tutorials ([jufo](https://jfp.csc.fi/jufoportaali) level 2). Chapters:
>    - Abstract
>    - I Introduction
>    - II Background (to the end of "B. Circuit Establishent for Tor HS")
>    - Fig. 6. Taxonomy for Tor attacks (Just the figure on page 2330.)
>  - Halonen, Ollikainen, Rajala 2023: [PhishSticks - The Ethical Hackers tool for BadUSB](https://www.youtube.com/watch?v=bDzVevtZiWE) (Video, about 3 minutes)

### Dingledine, Mathewson and Syverson 2004: Tor: The second-generation onion router. Chapter 3: 3 Design goals and assumptions
The document summarized can be read at the MIT website : [Dingledine, Mathewson and Syverson 2004: Tor: The second-generation onion router.](https://css.csail.mit.edu/6.858/2022/readings/tor-design.pdf)

The chapter details 

### Karunanayake, Ahmed, Malaney, Islam and Jha 2021: De-anonymisation attacks on tor: A survey
The document summarized can be read at the IEEE website : [Karunanayake, Ahmed, Malaney, Islam and Jha 2021: De-anonymisation attacks on tor: A survey](https://ieeexplore.ieee.org/ielx7/9739/9621320/09471821.pdf)

### Halonen, Ollikainen, Rajala 2023: PhishSticks - The Ethical Hackers tool for BadUSB
The video can be viewed at the YouYube website : [Halonen, Ollikainen, Rajala 2023: PhishSticks - The Ethical Hackers tool for BadUSB](https://www.youtube.com/watch?v=bDzVevtZiWE)

## a

>- a) Install TOR browser and access TOR network (.onion addresses). (Explain in detail how you installed it, and how you got access to TOR).

For this assignment, I used the [Arch Linux Wiki documentation](https://wiki.archlinux.org/title/Tor) on an updated system (-> *sudo pacman -Syu*)  
**Note** The wiki article is quite short if you only intend to install the tor browser, most of the article details on how to run a Tor service on you're computer.

So, let's install the Tor browser and test it.

![picture](/img/tor_browser-1.png)

Lets try a .onion address. but before that, we must connect to the tor network. Since I don't want to configure the connection, I'll just click *Connect* and we cant start surfing.

| Connect                                   | Connected                                 |
| ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-2.png)        | ![picture](/img/tor_browser-3.png)        |

Let's go to an .onion address, anyone will do for this part of the assignment, so let's go to the DuckDuckGo .onion.  
To get there, you do an empty search in the search field (next to the *Onionize* tab). Or more simply, just click on the search field and press enter.

![picture](/img/tor_browser-4.png)

**Note** It seems that DuckDuckGo can be used with Tor, but it only searches for clearnet addresses. To search for onion sites, we need to use another search engine. Tero mentioned previously Ahmia.fi, so we will continue with that one.

## b 

>- Browse TOR network.
>  - Find, take screenshots and comment
>    - search engine for onion sites
>    - human rights or civil rights organization
>    - marketplace
>    - fraud
>    - forum
>    - a well known organization (with regular postal addresses, offices or similar presence outside darknet)
>  - Use .onion addresses inside TOR network, not regular (clearnet) websites trough exit nodes.

### search engine for onion sites

Like previously mentioned, I tried ahmia.fi  
I appreciate that the tor browser offers a redirect to an .onion site if it's available.

![picture](/img/tor_browser-5.png)

The next sub-assignments will use ahmia as the search engine.  
All in all, the search engine looks, well what a search engine should I guess.  
All though I must note that the .onion site load quite slowly but selecting a new circuit for the site made the connection much faster.

### human rights or civil rights organization

Let's try amnesty international.

If found the amnesty site via blink list. 

| Ahmia search                          | Blinklist result                          | Amnesty .onion page                       |
| ------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-6.png)    | ![picture](/img/tor_browser-7.png)         | ![picture](/img/tor_browser-8.png)       |


### marketplace

We will continue our quest with ahmia. Let's just search with *marketplace* and see what we find.  
The first site seems to be Venus marketplace (*the best market place*?), let's see what it contains.  

| Search                                    | Venus marketplace                         |
| ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-9.png)        | ![picture](/img/tor_browser-10.png)       |

I seems that the offering is what you would expect maybe.

### fraud

Let's continue using ahmia and search for fraud.  
The fourth search result is *GothamCity*, this sounds promising, so let's go there :)  
It seems that the main offering is credit card frauds.  
Let's check the German offering.
25$ for a credit card, payable in Bitcoin or Monero.

| Ahmia search  for fraud               | GothamCity                                | German fraud offerings                    |
| ------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-11.png)   | ![picture](/img/tor_browser-12.png)       | ![picture](/img/tor_browser-13.png)       |

### forum

Once again, we will use ahmia to search for a forum.
The third result is *DarkNet Army - Carding and Hacking Forum*, sounds promising.
Let's select a thread, Avast Database leak seems interesting.
Well, well. The leak is old (2014) but I guess this qould be useful for some parties.

| Ahmia search for forum                | DarkNet Army site                         | Avast database thread                     |
| ------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-14.png)   | ![picture](/img/tor_browser-15.png)       | ![picture](/img/tor_browser-16.png)       |

### a well known organization (with regular postal addresses, offices or similar presence outside darknet)

I searched for a while with no luck, tried

- Meta
- MIT
- Harvard
- Linux foundation
- Amazon (I really don't know why i tried this)
- "a well known organization" (I felt desperate for a moment..)

Then I started thinking, a *legitimate* tor site would serve people who can't use them in the clearnet for some reason. New's could be something that might fall in to this category and it I found out hat BBC does offer this!

| Ahmia search for BBC                      | BBC tor sire                              |
| ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-17.png)        | ![picture](/img/tor_browser-18.png)       |

## c

>- Onion. In your own words, how does anonymity work in TOR? (e.g. how does it use: public keys, encryption, what algorithms? This subtask does not require tests with a computer.)

## d 

>- d) What kind of the threat models could TOR fit? (This subtask does not require tests with a computer.)

## e

>- Don't stick that stick. How does PhishSticks attack work? Would a typical organization be vulnerable? Does this link to a broader category of attacks and defenses? How could the risk be mitigated? (This subtask does not require tests with a computer.) (If you want, you can view [PhishSticks on Github](https://github.com/therealhalonen/PhishSticks/) and [PhishSticks Youtube channel.](https://www.youtube.com/@phishsticks_pentest/videos))

## f

>- f) Voluntary: I2P. Install and demonstrate use of I2P.

## g

>- g) Voluntary: Hyphanet. Install and demonstrate use of Hyphanet.

## h

>- h) Voluntary: Freenet. Install and demonstrate use of Freenet.

## i

>- i) Voluntary: GNUnet. Install and demonstrate use of GNUnet.

## j

>- j) Voluntary, difficult: Test PhisSticks USB HID attack on your own computer.