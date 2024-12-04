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

The chapter details the design goals and assumptions regarding the Tor, it also details factors that are not desired, marked as *Non-Goals* and possible threat models.

#### Design goals
The design goals fall in to four (4) categories.

- Deployability
  - Must be easy to run and implement in the real world with no unnecessary burden (high bandwidth for example), liability or high costs on the operator.

- Usability
  - Most not be difficult to use
    - Difficult systems will lead to fewer users and fewer users means less anonymity
  - Should be easily configurable with out the need to modify familiar applications and be available to common operating systems
  - Should not contain or introduce prohibitive delays on the users

- Flexibility
  - The protocol used must be flexible and well specified, so hopefully in the future the Tor design should not be reinvented.

- Simple design
  - The design should be well understood, simple and stable, containing proven and well understood security parameters.
  - Additional features might add complexity
  - Should not contain unproven techniques to compromise the design

#### Non-goals for the design

The non-Goals are stated to be

- Not peer-to-peer
  - The peer-to-peer model in decentralized environments still have many problems.
  - Could be scaled with thousands of short lived servers, but these might be controlled my adversaries

- Not secure against end-to-end attacks
  - Tor does not intend or claim to be completely secure regarding end-to-end timing or intersection attacks

- No protocol normalization
  - Tor does not offer protocol normalization
  - If users seeks anonymity from responders, they should layer Tor by using a filtering proxy

- Not steganographic
  - Tor does not try to conceal who is connected to the network

#### Threat model

- The Tor design assumes that a global passive adversary can observe at least some fraction of the network traffic
  - The Tor design does no assume that it can protect the entirety of the network from attack
  - Tor aims to prevent the adversary from analyzing the traffic patterns, so the adversary could not learn on which point of the network they should attack

- The global passive adversaries aim is typically to observe both ends of the encrypted communication (initiator and the responder) to confirm traffic between two parties.

- The possible adversary can employ multiple ways to try and compromise the network, these can include compromising routers or private keys, denying service (DoD) from trusted servers, with the aim to divert traffic to compromised servers, attacking nodes to decrease the networks readability.


### Karunanayake, Ahmed, Malaney, Islam and Jha 2021: De-anonymisation attacks on tor: A survey

The document summarized can be read at the IEEE website : [Karunanayake, Ahmed, Malaney, Islam and Jha 2021: De-anonymisation attacks on tor: A survey](https://ieeexplore.ieee.org/ielx7/9739/9621320/09471821.pdf)

Tor is most likely the most used low latency, anonymous network in use today. This attracts legitimate users and services who seek to to stay hidden in their activities, these could include governmental organizations, like intelligence services or the military, people who can not use the web freely in their own country or location or are afraid of the implications if they do so. This also attracts parties that act in unlawful businesses, criminals. This is why de-anonymisation of the Tor network, it's users and services, has been of great interest to law enforcement and intelligence agencies alike.

The Tor network is defined as a Overlay network (network that operates on top of another network, the Internet in this case) that is based on the Transmission Control Protocol (TCP, OSI layer 4) to establish a secure network connection between client and server. This system requires voluntary relays (nodes) to create a circuit (entry, middle and exit), which makes the secure connection possible.
The Tor network consists of several key components:

- Onion Proxy (OP)
  - Software that is installed on the client device and is used to access the Tor network
- Directory Servers (DS)
  - Trusted servers that that offers the Onion Proxy details of the TOR network status. I see these as DNS servers in a way, since they help the OP in navigating the Tor network
  - The OP selects three relays (nodes) from the DS to use in establishing a connection.
- Entry Node/Guard
  - First entrypoint to the Tor network from the OP. It knows the IP address of the client, which can cause security concerns if the node is compromised, this is why  Guard nodes are used. These are considered trusted by the DS.
- Exit Node
  - This is the last node on the way to the server and because of this, it knows the servers IP. Like with the entry node, if this is compromised, traffic can be monitored (if not using encryption, like TLS) and the location of the Server is compromised.
- Hidden Services (HS)
  - These are the services (servers) using a .onion address, also known a onion services.
- Introduction Points
 - Random nodes selected by the Hs to act as entry point to the service, usually the HS selects several entry points to avoid denial of service attacks (DoD). a three hop circuit is established (entry, middle, exit) with the server and the Introduction Point, this way the entry points don't know the IP address of the server. The introduction points is advertized to the DS and the OP can use these to access the hidden services.
- Rendezvous Points (RPs)
  - A random node selected by the OP to connect with the introduction points. a circuit (entry, middle, exit) is formed in the connection, to hide the clients IP address
- Bridges
  - Nodes that are no listed in the DS. Adds redundance and security to the network, since they are not advertized by the DS
  - They replace guard nodes in the circuit

The document details how a connection circuit is made and how circuit is established with a hidden service.

The circuit connection is made when the onion proxy (OP) contacts the a directory server (DS) and receives a list of active relays. Th OP selects three relays for the circuit to acts as the entry, middle and exit node. Note that the entry node is a trusted guard node. Public keys are exchanged with the three nodes, one node at a time with corresponding nodes.

To access a hidden service, the onion proxy must find the introduction point for that service from the directory server. After this the onion proxy establishes a circuit with a rendezvous point. The onion proxy then sends a message, informing of a connection request to the introduction point vie the rendezvous point, the introduction point forwards said message to the hidden service server. If the hidden service accepts the connection request, it forms it's own circuit with the rendezvous point and communicates with the onion proxy.

The documents also presents a taxonomy for Tor related attacks.  
These attacks fall in four (4) categories, with one having four (sub categories) 

1. Network disruption
2. De-anonymisation
    1. Side channel
    2. Entry and exit routers
    3. OP/OR/server
    4. Hybrid
3. Censorship
4. Generic

As stated in the introduction, de-anonymisation attacks represent the larges segment in these.  
These attacks fall in two categories, active and passive.

### Halonen, Ollikainen, Rajala 2023: PhishSticks - The Ethical Hackers tool for BadUSB

The video can be viewed at the YouTube website : [Halonen, Ollikainen, Rajala 2023: PhishSticks - The Ethical Hackers tool for BadUSB](https://www.youtube.com/watch?v=bDzVevtZiWE)

In the video Halonen, Ollikainen and Rajala demonstrate their BadUsd dropper in a simulated corporation settings, to showcase dangers in using unknown USB devices.  
A malicious USB stick is left for the CEO actor to inject to his computer, which when installed installs a keylogger the system. It should be noted that the BadUsb can be packaged with different payloads.

The video describes the process well

1. In insertion the the malicious payload injects keystrokes and download the malware using powershell
2. The malware, keylogger in this case, records user input and saves them
3. The compromised data is send to the attacker via HTTP post
4. the malware removes the logged data after sending it, clearing traces of it's activities.

I've seen this video before and find it very informative and entertaining. The results were very goog, considering this was done by bachelor student's on their school project that last only 16 weeks.

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

Then I started thinking, a *legitimate* tor site would serve people who can't use them in the clearnet for some reason. New's could be something that might fall in to this category and it I found out that BBC does offer this!

| Ahmia search for BBC                      | BBC tor sire                              |
| ----------------------------------------- | ----------------------------------------- |
| ![picture](/img/tor_browser-17.png)        | ![picture](/img/tor_browser-18.png)       |

## c

>- Onion. In your own words, how does anonymity work in TOR? (e.g. how does it use: public keys, encryption, what algorithms? This subtask does not require tests with a computer.)

## d 

>- d) What kind of the threat models could TOR fit? (This subtask does not require tests with a computer.)

## e

>- Don't stick that stick. How does PhishSticks attack work? Would a typical organization be vulnerable? Does this link to a broader category of attacks and defenses? How could the risk be mitigated? (This subtask does not require tests with a computer.) (If you want, you can view [PhishSticks on Github](https://github.com/therealhalonen/PhishSticks/) and [PhishSticks Youtube channel.](https://www.youtube.com/@phishsticks_pentest/videos))  

For the sake of convenience, I'll split this to separate questions and answers.

>How does PhishSticks attack work?

In the PhishSticks attack the USB acts as an human interface device (HID), these are for instance keyboards and mice.
In short, the USB firmware is manipulated to execute malicious code, once inserted.
In this case the malicious code injects keystrokes to open Windows Run prompt and download the malware using powershell. In essence, the supposed "keyboard" is typing this.

Computers are vulnerable for this because, well, USB devices are common and they need to work without issues.
The computer recognizes the USB device by USB Enumeration, this tells the computer what device is connected and what drivers it needs to operate.
In this case, if the USB stick tells it's a keyboard, the computer can't tell the difference.  

>Would a typical organization be vulnerable?

Yes they are, I could not see why not. USB sticks are so common and it only needs one person to injected the device to compromise the company.
In the PhishSticks example, this of course assumes the company is a Windows company. Apple and Linux are not affected by powershell, "Windows button" + R or .LNK files, but then again Windows is still the dominant desktop operating system, I would guess that the percentage is like 80% of desktops, so I would say that constitutes the most cost effective segment to target.

>Does this link to a broader category of attacks and defenses?



>How could the risk be mitigated?

I reviewed the PhishSticks GitHub page and they list an admirable list of mitigations for this kind of attack. These include:

- **Don't plug in an unknown device!!**
- Disable powershell for users
  - This is a good one, because the average users most likely doesn't need this.
- Disable the Windows run prompt ("Windows button" + R)
  - Another reasonable mitigation, users seldom need this.
- Disable the use of removable devices
  - This is maybe a bit heavy handed approach, but very effective is needed.

One tip from my hat, do you need you're USB ports? If not, then just block them with hot glue. This is of course a permanent solutions but also very effective in preventing from using USB devices.

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