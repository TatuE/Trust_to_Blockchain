# h7 Free science

>*Follow the new science in infosec.*
>
>*Jump right into high-quality articles. Save your time, read review article first.*

## Assignment

Full assignment can be viewed at the courses website : [h7 Free science!](https://terokarvinen.com/trust-to-blockchain/#h7-free-science)

## x

>- x) Read and summarize (briefly, e.g. with some bullets)
>   - Infosec review article.
>     - Review: Find review article on an infosec area you're interested in
>     - Peer reviewed: Pick an article on published on a journal that has JUFO rating 1, 2 or 3.
>     - Fresh: Prefer fresh articles, less than 2 years old.

For this assignment I ended up looking at the subject with a broader scope in information security, instead of looking for a specific subjects like "brute force attack" or "BadUSb". The keywords in searches that were used are : "security vulnerability" and "information security". I think that the articles I found were quite interesting and enlightening.

### Article summary, review article

I know that this article is a bit old, but the name was so appealing I chose it for this assignment.

Article reviewed : Lightweight cryptography algorithms for internet of things enabled networks: An overview.

- **[Shamala, L.M., Zayaraz, G., Vivekanandan, K. and Vijayalakshmi, V., 2021. Lightweight cryptography algorithms for internet of things enabled networks: An overview. In Journal of Physics: Conference Series (Vol. 1717, No. 1, p. 012072). IOP Publishing.](https://iopscience.iop.org/article/10.1088/1742-6596/1717/1/012072/meta)**

IoT (Internet of Things), devices have become increasingly common in the world. Globally there is a network of billions of small devices that can communicate with one another and in most cases, can connect to the internet to communicate with servers and applications. Often they are low power devices, which makes them cheap and convenient to deploy for various purposes, like traffic control, environmental surveillance and home automation. 

Cause IoT devices are often designed to be low power devices, with limited processing capabilities and storage space, security considerations are often superseded by operational requirements. Unsecure IoT devices offer a gateway for an attack and is often an easy target for hackers, if the device is Unprotected and vulnerable.  
Efficient end-to-end encrypted communication is needed, but conventional cryptography methods are not suitable for these devices do to the resource limited design of th devices and to increase the security of IoT devices, a suitable method for cryptography should be implemented to ensure that data integrity and privacy could be achieve, with the least effect on device performance. The is a method for this, Lightweight cryptography (LWC), which is used in internet security protocols to provide adequate security.

Design challenges regarding hardware and software implementations affect the decisions on a suitable lightweight cryptography algorithm. Memory and energy consumption, processor clock cycle and code size are the primary metric to determine the feasibility of a cryptography algorithm, because of restricted requirements, symmetric ciphers are more more ideal for this.
The study tested 12 different lightweight ciphers to determine which of them would prove to be efficient for IoT device security.

The tested algorithms were:

- Chaskey
- Chaskey-LTS
- SPECK
- SIMON
- LEA
- RECTANGLE
- SPARX
- AES
- RC5-20
- HIGHT
- Fantomas
- Robin

The test results made evident, that Chaskey was the best performing algorithm, if a metrics were considered. 

In designing IoT security, balancing the security, cost, and performance is a difficult task. The goal is to reach a level of  acceptable security, with limited impact on the device performance.

#### Personal thoughts of the article

This was the last article I reviewed of these three, and honestly I should have reserved more time writing it. The article it self was highly interesting. I did an IoT project in Haaga-Helia in 2018 utilizing LoRa for communication, with a LoRa receiver  connected to the internet and secure communication was an aspect that was thought off, but never implemented in the PoC. Lightweight cryptography as a concept is actually new for me, and I could imagine that I will be coming back to this subject in the future.

### Article summary, peer reviewed

Article reviewed : How memory anxiety can influence password security behavior.

- **[Woods, N. and Siponen, M., 2024. How memory anxiety can influence password security behavior. Computers & Security, 137, p.103589](https://www.sciencedirect.com/science/article/pii/S0167404823004996?via%3Dihub).**
  - Jufo portal: [Computers and security (JuhoID : 53963), jufo level 2](https://jfp.csc.fi/jufoportaali?Jufo_ID=53963)
  - Tiedotutkimus.fi page : [How Memory Anxiety Can Influence Password Security Behavior](https://tiedejatutkimus.fi/fi/results/publication/0684013524)

The way people handle passwords is becoming increasingly important, with the increasing amount of digital services used at home and at work.  
The behavior of reusing or modifying passwords or using external memory storage mediums, like writing them down on a piece of paper, has become increasingly prevalent and poses a security threat, if passwords have been compromised from even one service.

People usually resort to risky password handling behaviors because of the overwhelming amount of different passwords they are required to remember. In many cases this is do to the persons fear that they are unable to remember all their passwords, which should be unique for every service. Security policies often mandate that the same passwords cannot be reused, which leads to people modifying their passwords. This usually means making small changes to the old password to make it more memorable, in most cases the modifications are made to the beginning and/or end of the old password, which makes it vulnerable if another passwords is leaked created in the same manner. Some people are unaware that this behavior poses a risks, but some continue this practice even after made aware of the security considerations, because they see no alternative to the situation or they feel that the risks are not that serious.

It has been proposed, that metamemory attributes could help explain risky password handling behaviors, why people have insecurity in their ability to remember different passwords.  
Metamemory is the persons believe in their faculties related to their memory capabilities.  
Metamemory is presented in seven scales.

1. Strategy - Knowledge and ability to use memory strategies and techniques.
2. Task - Knowledge and understanding of basic memory processes.
3. Capacity - Beliefs and perceptions about our own memory capacities.
4. Change - Perception of the change in our own memory capabilities.
5. Anxiety - Anxiety towards our memory performance and/or the perception of the relationship between anxiety and memory performance.
6. Achievement - Perception of our motivation to perform well in memory tasks.
7. Locus - Perception of our level of control over our own memory skill.

Two studies were performed which measured the participants metamemory in managing their passwords. The study used a modified version of the Metamemory In Adulthood (MIA) questionnaire, which was first introduced in 1988. The results were analyzed to find a relationship between password reuse and modification in relation to password metamemory.
The study results pointed out that from the seven scales of metamemory, anxiety was the only one that played a predictive factor in password reuse and modification in both studies.

Passwords and password security holds an important place in our everyday lives, at work and also at home.  
They protect valuable assets and because of this, many people develop fears that they will forget their passwords, which would cause inconvenience in accessing their services or even prevent the access, consequently this fear increases anxiety which often hinders their trust on their own capability to remember their passwords. This anxiety often leads to insecure password practices, which often pose a security risk for the person and the organization they are working for.

When we better understand how anxiety can affect the persons security behaviors, we can try to implement better teaching and training methods that takes this in to consideration.

#### Personal assessment of the article

When I found this article I first considered it to be a bit off topic, but when examined, I found it to be insightful.  
To better improve security practices, we must try to understand what aspects to consider, so insecure practices could be successfully remedied.  
The relation of anxiety in insecure password handling was a new approach to me, but after reading the article made sense to me.  
This of course could remedied by using a password manager to store the users passwords but in my opinion that would solve only half of the problem.  
Finding a technical solution for a behavioral problem, does not entirely fix the underlying problem with the peoples tendency implement bad password practices, it will help to improve the situation non the less. 

### Article summary, Fresh

Article reviewed : Cyber security: State of the art, challenges and future directions.

- **[Admass, W.S., Munaye, Y.Y. and Diro, A.A., 2024. Cyber security: State of the art, challenges and future directions. Cyber Security and Applications, 2, p.100031.](https://www.sciencedirect.com/science/article/pii/S2772918423000188)**

The modern society has experienced rapid increase of a internet connected device and services. Cyber security has become a increasingly critical concern that needs the attention of researchers, governments and organizations to ensure the safety, reliability and redundancy of information systems by ensuring security measures that will prepare, protect and mitigate against a possible attack by hostile adversaries. Societies and organizations have recognized the growing threat of cyber threats, and are responding by drawing new guidelines, frameworks and legislation to combat this threat to ensure data protection and privacy.

cyber security threats affect many aspects of our local and global information systems. Since more an more devices are connected to the internet, the field for attack has increased to include vital components in our society. These include:

- Smart grids - Enhancement of the power grid that connects the grid and power stations to a network
- Vehicular communications - Transportation vehicles are becoming smarter and more depended on vehicle-to-vehicle communication over the network.
- Smart cities - Municipal utilities like water, power, transportation and communication devices like IoT sensors and camera are increasingly relying on network connection too operate efficiently.
- Smart eHealth systems - Network connected IoT based devices that provide patient monitoring and medical devices are becoming the standard and are increasing in numbers.

Cyber attacks have become very sophisticated and employ advanced techniques for exploit. APT:s (Advanced persistent threat) pose an increasing threat, they are organized and employ targeted campaign to endanger governmental, military and large private industry entities to gain economic, political, and/or strategic advantages.  

The nature of cyber threats are continually evolving and consideration should be applied to secure and defend against this evolution.

- Zero-day attacks, that exploit unpatched vulnerabilities that may or may not be known, are challenging to protect against with the increasing number of interconnected systems.  
- IoT devices have been and still are a concern to security, they can utilize poor encryption and data management methods, and often rely on wireless communication that makes them eavesdropping, interception, or data modification.  
- Using AI and machine learning can give the threat actors an increasing advantage to learn and exploit weaknesses.  
- Cloud computing is convenient for organizations to adopt but increases the complexities and challenges in security. Insecure API:s and shared infrastructure can provide weak point to an attack and organizations may experience data breaches and unauthorized access incidents.

Cyber security is an evolving field, that can provide opportunities to combat the growing threat and bring forth new difficulties.

- AI and machine learning can be utilized to defend against security threats, they can aid in detecting weaknesses and threats by analyzing a vast amount of data. This could be applied to security automation and automation frameworks that would be able to respond quickly to a threat scenario.

- Measures should also be taken to share the knowledge of attacks and attack patterns to increase collaboration and awareness in the cyber security field.

- Human-centric security measures is essential and should be applied to improve the awareness and education related to cyber security. 

- Biometric authentication can increase security, when combined with standard authentication techniques such as passwords.

- The promise of Quantum computing creates a serious threat to the integrity of modern cryptographic algorithms and in the future, measures should be made to try and implement a quantum-resistant cryptography technology to ensure privacy and long term security.

- Blockchain technology could help in ensuring data integrity and security, in areas like transactions and supply chains.

#### Personal assessment of this article

All in all the article was good, but honestly not that enlightening, especially after this course.  
It brought up valid concerns on security and proposed interesting solutions to develop, I know that AI and machine learning is bit over hypes now, this article detailed sound applications to apply them to.

## a

>- a) Voluntary: Create an alert that sends you new peer reviewed articles >on your area of intrest. Once it's working, remember to make the filter >tighter, so that you're happy when you recieve a message.

I created alerts for subjects I found interesting. I'm quietly trying to distance my self from google, that's why I'm using my school gmail account.

![Picture](/img/google_scholar_alerts.png)