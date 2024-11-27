# h5 HelSec

## Assignment

Full assignment can be viewed at the courses website : [h5 Helsec](https://terokarvinen.com/trust-to-blockchain/#h5-helsec)

## x

>x) Watch and summarize. Add your own comments, ideas and questions.
>
>- Two full length HelSec presentations on the November event (2024-11-21 w47 Thu)
>- Voluntary bonus: A third one.
>- Twitch stream for HelSec

### Helsec November 2024 Meetup

I managed to get a ticket for his event and ended up staying through the whole event (expect for the networking and after party, it was a normal weekday after all).

All in all the event was quite pleasant. The venue was excellent for the purpose, there was even free pizza, for which I did not partake since I was tourist, let's not start free loading from the get go.

So, on with the assignment, the show and the presentations. Since I was sitting at a nice spot in the audience, I ended up taking *a lot* of photos.
I must be honest, I sat through the whole show and didn't managed to ask any questions from the presenters...

| Event start                               | Event schedule                            |
| ----------------------------------------- | ----------------------------------------- |
| ![Picture](/img/HelSec/IMG_0896.JPEG)     | ![Picture](/img/HelSec/IMG_0897.JPEG)     |

### Jos Helmich - Industrial Cyber Security

| Topic                                     | Bio                                       |
| ----------------------------------------- | ----------------------------------------- |
| ![Picture](/img/HelSec/IMG_0899.JPEG)     | ![Picture](/img/HelSec/IMG_0901.JPEG)     |

Jos works as a product security architect at Konecranes and is a member of the [ENISA](https://www.enisa.europa.eu/) (*European Union Agency for Cybersecurity*) advisory group.
He has over thirty years of experience in software engineering.
Jos started the presentation by talking about ENISA and it's role on providing guidance and council for EU regulations and directives regarding cybersecurity. I appreciated that pointed out and made a clear distinction between EU regulations and directives.

- Regulation => Law that must be applied in its entirety across the EU.
- Directive => A legislative goal that EU member states must achieve, methods on how to archive this often varies between member states.

Jos also talked about US and EU legislative difference on cybersecurity. The united states being a big player globally necessitates that these must be considered if planning on doing business in the US market. In short, the US is a combination of presidential executive orders (which might be temporary), policies and federal laws in contrast to the EU, which is unified on regulations and directives.

The work of ENISA can be seen in regulations like the GDPR and directives like NIS-2, CRA and AIA.

- GDPR (General Data Protection Regulation)
  - Data protection law emphasizing information privacy for EU citizens
  - Effectively in force since 2018
- NIS-2 (Network and Information Security directive)
  - Deals with corporate responsibilities regarding cybersecurity
  - In force since 2023
- CRA (Cyber Resilience Act)(Regulation)
  - Details rules regarding digital products and their security
  - Obligates the support for the entire product life cycle
  - Adopted in 2024, Effectively forced in 2027.
- AIA (Artificial Intelligence Act)
  - a legal framework for the use of AI in EU.
  - Details four risk levels based on the intended use:
    - Unacceptable risk : Social scoring, state monitoring
    - High risk : Medical devices, recruitment
    - AI with specific transparency obligations (I would use the word *limited*, since it goes nicely with thw others) : Impersonation ("bots")
    - minimal or no risk
  - Adopted in 2024

To be honest, GDPR was mentioned in the presentation, but most of the taking was about NIS-2, CRA and AIA.

I liked the comparison that Jos made with NIS-2 and CRA.

| NIS-2                                     | CRA                                       |
| ----------------------------------------- | ----------------------------------------- |
| Keep you're house in order                | Don't mess up someone else's house        |
| Don't buy bad or defective stuff          | Don't sell bad or defective stuff         |

Jos's presentation was good and I brought up legal considerations in a form that was easy to follow and interesting. I must give five cheers and a hurrah for the network topology in secure manufacturing that was presented. Funny fact, the two ENISA members from Finland are not Native Finns. One is Dutch and another is from Germany.

### Jos's presentation photo gallery

|                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |
| ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- |
| ![Picture](/img/HelSec/IMG_0902.JPEG)             | ![Picture](/img/HelSec/IMG_0903.JPEG)             | ![Picture](/img/HelSec/IMG_0904.JPEG)             | ![Picture](/img/HelSec/IMG_0905.JPEG)             | ![Picture](/img/HelSec/IMG_0906.JPEG)             | ![Picture](/img/HelSec/IMG_0907.JPEG)             | ![Picture](/img/HelSec/IMG_0908.JPEG)             |
| ![Picture](/img/HelSec/IMG_0909.JPEG)             | ![Picture](/img/HelSec/IMG_0910.JPEG)             | ![Picture](/img/HelSec/IMG_0911.JPEG)             | ![Picture](/img/HelSec/IMG_0912.JPEG)             | ![Picture](/img/HelSec/IMG_0913.JPEG)             | ![Picture](/img/HelSec/IMG_0914.JPEG)             | ![Picture](/img/HelSec/IMG_0915.JPEG)             |

### Heikki ”zokol” Juva - State of Union

| Topic                                     | Bio                                       |
| ----------------------------------------- | ----------------------------------------- |
| ![Picture](/img/HelSec/IMG_0916.JPEG)     | ![Picture](/img/HelSec/IMG_0917.JPEG)     |

In my opinion Heikki's presentation was the most interesting one of the three, since it was quite hands on.
Heikki is a tech lead and a hardware engineer at Traficom. He is aldo the man behind the DisObey badges.
His presentation started with introducing RED (Radio Equipment Directive) and CRA (Cyber Resilience Act), after that he presented on how he tested consumer devices on how they meet these requirements.

Small brief on the acronyms:

#### CRA (Cyber Resilience Act)

- This was discussed earlier in Jos's presentation
- Tries to fix the inadequate level of cybersecurity in many products, or inadequate security updates to such products and software.
- Effectively forced from 2027 onwards.

#### RED (Radio Equipment Directive)

- Places a regulatory framework for radio equipment
- Sets requirements for network security (18031-1), protection of personal information of the users (18031-2) and protection from financial misuse (18031-3).
- Effectively forced from 2025 onwards.

Heikki also presented a very nice diagram he made evaluating RED

![Red evaluation Picture - Copyright Zokol - Heikki Juva](https://github.com/Zokol/RED-CRA/blob/main/RED%20evaluation.png)

According to Heikki Red has good and also problematic requirements.

- Good requirements are:
  - input validation
  - vulncoor (vulnerability coordination and reporting to centralized authorities)
  - cryptographic best practices
  - strong passwords or the possibility for user to change password and content filtering (loading content form trusted sources only).

- The problematic requirements are:
  - compatibility (backwards compatibility may necessitate less secure solutions)
  - Outsourcing security
  - Paper-based evaluation

#### Practical evaluation of RED with consumer devices

For device testing Heikki presented a research question

- *How do consumer devices communicate?*
  - *To where?*
  - *Why?*

He talked about testing different consumer devices and how they evaluate under RED. The prerequisite for the selected devices were:

- Must be available in the consumer-market
- Needs to connect to the Internet
- Are popular in the market, in the top 10 of the device category.

The evaluation was done by blackbox testing, meaning that the device manufacturer was not consulted or the test was not performed by the device manufacturer. He noted that if you have manufacturer support (technical documents etc.) then there are better options for blackbox testing.

The devices are classified in accordance with the RED requirements.

- Does the device connect to the network? -> If so,  network security (18031-1) requirement applies
- Does the device collect personal information? -> If so, protection of personal information of the users (18031-2) requirement applies
- Does the device handle financial assets? -> If so,  protection from financial misuse (18031-3) requirement applies

#### Practical example of a device evaluation

To demonstrate blackbox evaluating a device using RED, Heikki presented the *Case Eagle* a baby monitoring camera that can be used by an iOS or Android application and it connects to the internet. It included a camera with a microphone and a wireless heartrate monitor band. By classifying this device it must apply to RED requirements 18031-1 and 18031-2.

As an extra, Heikki demonstrated how he could read device information from the baby monitor after it was supposedly been factory reset. Heikki could extract passwords, ip-addresses and information on the original author.

#### Over all analysis of tested devices

In testing it was found out that most of the devices communicated securely, they used TLS version 1.3, encrypted the content and used certificate pinning (a security measure that links a host with their expected digital certificate).Since they are IoT devices by nature, to save energy they don't stay connected to the network all the time, establishing connection when needed. This makes monitoring testing a bit harder. Most of the devices were programmed on *baremetal*, so they don't have a operating system installed on them.

Most concerning problems were that device or user data could not be removed from the device(by factory resetting)(With baremetal devices this was read directly from the NAND flash chip) or the uncertainty that personal data has been removed from the cloud, sharing private information with third parties and a complex chain in manufacturing (sellers don't know how the device works).





### Heikki's presentation photo gallery

|                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |
| ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- |
| ![Picture](/img/HelSec/IMG_0918.JPEG)             | ![Picture](/img/HelSec/IMG_0919.JPEG)             | ![Picture](/img/HelSec/IMG_0920.JPEG)             | ![Picture](/img/HelSec/IMG_0922.JPEG)             | ![Picture](/img/HelSec/IMG_0923.JPEG)             | ![Picture](/img/HelSec/IMG_0924.JPEG)             |                                                   |                                                   |
| ![Picture](/img/HelSec/IMG_0925.JPEG)             | ![Picture](/img/HelSec/IMG_0927.JPEG)             | ![Picture](/img/HelSec/IMG_0928.JPEG)             | ![Picture](/img/HelSec/IMG_0929.JPEG)             | ![Picture](/img/HelSec/IMG_0931.JPEG)             | ![Picture](/img/HelSec/IMG_0932.JPEG)             | ![Picture](/img/HelSec/IMG_0933.JPEG)             | ![Picture](/img/HelSec/IMG_0934.JPEG)             |
| ![Picture](/img/HelSec/IMG_0935.JPEG)             | ![Picture](/img/HelSec/IMG_0936.JPEG)             | ![Picture](/img/HelSec/IMG_0937.JPEG)             | ![Picture](/img/HelSec/IMG_0938.JPEG)             | ![Picture](/img/HelSec/IMG_0940.JPEG)             | ![Picture](/img/HelSec/IMG_0941.JPEG)             | ![Picture](/img/HelSec/IMG_0942.JPEG)             | ![Picture](/img/HelSec/IMG_0943.JPEG)             |

### Joona "Rinorragi" Immonen - My experiences on Defender External Attack Surface Management

| Topic                                     | Agenda                                    |
| ----------------------------------------- | ----------------------------------------- |
| ![Picture](/img/HelSec/IMG_0944.JPEG)     | ![Picture](/img/HelSec/IMG_0947.JPEG)     |

| Bio                                       | Microsoft *qualifications*                |
| ----------------------------------------- | ----------------------------------------- |
| ![Picture](/img/HelSec/IMG_0945.JPEG)     | ![Picture](/img/HelSec/IMG_0946.JPEG)     |

Joona's presentation was very entertaining, he was a very good presenter and quite often the audience laughed at his comments. Joona has worked in IT for over twenty years, of which the last eight has been in cyber security.

His presentation was about his experience with Azure EASM (External Attack Surface Management) offered by Microsoft and how it compares with an open source alternative OWASP AMASS.

### Joona's presentation photo gallery

|                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |                                                   |
| ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------- |
| ![Picture](/img/HelSec/IMG_0949.JPEG)             | ![Picture](/img/HelSec/IMG_0950.JPEG)             | ![Picture](/img/HelSec/IMG_0951.JPEG)             | ![Picture](/img/HelSec/IMG_0952.JPEG)             | ![Picture](/img/HelSec/IMG_0953.JPEG)             | ![Picture](/img/HelSec/IMG_0954.JPEG)             | ![Picture](/img/HelSec/IMG_0955.JPEG)             |
| ![Picture](/img/HelSec/IMG_0956.JPEG)             | ![Picture](/img/HelSec/IMG_0957.JPEG)             | ![Picture](/img/HelSec/IMG_0959.JPEG)             | ![Picture](/img/HelSec/IMG_0960.JPEG)             | ![Picture](/img/HelSec/IMG_0961.JPEG)             | ![Picture](/img/HelSec/IMG_0963.JPEG)             | ![Picture](/img/HelSec/IMG_0964.JPEG)             |
