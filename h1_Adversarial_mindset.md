# h1 Adversarial mindset

>*You will read the famous cyber kill chain paper. And start your very own hacking lab by installing Linux virtual machine.*
>*You can only start this homework after accepting course rules in Moodle.*

## Assignment

>x) Read and summarize. Some bullets is enough for a summary.
>- Hutchins et al 2011: Intelligence-Driven Computer Network Defense Informed by Analysis of Adversary Campaigns and Intrusion Kill Chains
>- Darknet Diaries. Pick one episode. (RSS feed)
>- MITRE ATT&CK FAQ explains the ATT&CK Enterprise Matrix. Explain "tactic", "technique" and "procedure" in context of ATT&CK, and give an example of each. The enterprise matrix is big, you can just glimpse/browse it to see what's available instead of reading hundreds of pages.

>a) How would you compare Cyber Kill Chain and ATT&CK Enterprise matrix? Who do you think could benefit from these models?

>b) Pick a security incident and learn about it. Write briefly about it. Point out the concepts of threat actor, exploit, vulnerability and (business) impact. (You can find writeups about security incidents from Darknet Diaries and Krebs)

>c) Install Debian on Virtualbox. Report your work, including the environment (including host OS, the real physical computer used), the steps you took and their results.

>d) Voluntary bonus: Use either (Hutchins et al 2011) cyber kill chain or MITRE ATT&CK framework for analyzing a security incident. You can pick any incident you want (even the one you used earlier in this homework), but try to pick a source that gives you enough technical and business detail to do some analysis. (If you're in a hurry, cyber kill chain is much simpler. If you're technically skillful, you might find ATT&CK interesting)

>e) Voluntary bonus: What do you consider the fundamentals of security? What are the theoretical foundations you would teach on the first day?

>f) Voluntary bonus: Do you think anything is missing from these models, Cyber Kill Chain or MITRE ATT&CK?

### Tips:
>- Some bullets for each article is enough. You don't need to have all content of the long articles in your summary.
>- For the summary, add your own question, idea or comment
>- Hutchins et. al. is the cyber kill chain paper.
>- Darknet diaries: you'll probably have a different episode from everyone else, as long as you don't take the latest episode.
>- To listen to podcasts on Android, you can use AntennaPod from F-Droid or Google Play
>- Refer to each source you've used: the course, the task given, the papers, the podcasts - all sources you've used. All sources must be mentioned in every document, page or blog using them. It's enough to just name and link them, you don't need to write another list in the end. In fact, it's important to know which information comes from which source.
>- My article Install Debian on Virtualbox explains it pretty well.
>- Got stuck with VirtualBox or Linux? Don't worry, computers are like that. Write a detailed report (in your homework) with screenshots. Explain what approaches you took and what happened. List where you found advice or articles. Explain your ideas why it would not work. You'll get help and advise in the class (and that's not all - you'll also get Linux on your virtual machine).

## Assignment answers

### x

#### Hutchins et al 2011: Intelligence-Driven Computer Network Defense Informed by Analysis of Adversary Campaigns and Intrusion Kill Chains

The document summarized can be read at the Lockheed Martin website : [Intelligence-Driven Computer Network Defense
Informed by Analysis of Adversary Campaigns and
Intrusion Kill Chains](https://lockheedmartin.com/content/dam/lockheed-martin/rms/documents/cyber/LM-White-Paper-Intel-Driven-Defense.pdf)

- Introduces the concept of an Intrusion Kill Chain, to engage and mitigate intrusions by APT:s (Advanced Persistent Threat).
    - APT:s are a new class of security threat that are manually operated and more focused, they poses a more complex threat than threats deemed more "traditional", such as worms and viruses.   
- The Intrusion Kill Chain consists of seven (7) phases used by the attacker in sequence:
    1. Reconnaissance
    2. Weaponization
    3. Delivery
    4. Exploitation
    5. Installation 
    6. Command and Control (C2)
    7. Actions on Objectives
    - If an intrusion is prevented within a kill chain phase, the attacker cannot continue to the next phase.
- The Intrusion Kill Chain drives to be more proactive in mitigating intrusions by utilizing Intelligence-driven computer network defense (intelligence-drive CND) analyzing the compromised phase (if detected) of the attack but more importantly the previous phases within the the kill chain.
    - Analyzing and determining indicators for the attack.
        - Indicators can be atomic (indivisible information, like an IP-address), computed (collected data from the intrusion, like hash values) and behavioral (collection of atomic and computed values which form a logical structure).
    - Applying a mitigation for the affected phase and possibly the phases before and after, if if the indicators analyzed enable this.
    - Moving the possibility of detection to earlier phases of the kill chain.
    - Finding similarities between attacks based on discovered indicators, applying mitigation if possible and forming a coherent picture a possible intrusion campaign.
    - Aiming to be a step ahead of possible attackers and increasing the costs to benefit ratio for the attacked.

### Darknet Diaries, episode 83: NSA Cryptologist

I chose this episode randomly, since the last episode is numbered at 150, I would assume the first is 1. The RANDOM bash function decided for me (*echo $((1 + $RANDOM % 150))*). R
As far as the name goes, I'm pleased with the result. The episode can be found at the Darknet Diaries website : [EP 83: NSA Cryptologists](https://darknetdiaries.com/episode/83/) 

#### Episode summary

### ATT&CK Enterprise Matrix

The documents summarized can be read at the Mitre Corporation website: [Mitre ATT&CK FAQ](https://attack.mitre.org/resources/faq/), [Mitre ATT&CK Enterprise Matrix](https://attack.mitre.org/matrices/enterprise/)

- A large, bi-annually updated knowledge repository, that consists of tactics and techniques used by cyber security threat actors. created and maintained by the Mitre Corporation.
- Consists of two parts:
    - ATT&CK for Enterprise, focuses on enterprise IT-networks and cloud applications.  
    - ATT&CK for Mobile, focuses on mobile devices.
- Consists of techniques for multiple systems and platforms, these include Windows, Linux, macOS, Android and iOS to name a few.
- Focuses on tactics, techniques, and procedures (TTP:s) used by APT:s (Advanced Persistent Threat). [APT explained in the cyber Kill chain](h1_Adversarial_mindset.md#hutchins-et-al-2011-intelligence-driven-computer-network-defense-informed-by-analysis-of-adversary-campaigns-and-intrusion-kill-chains).
    - Tactics
        - Represents the "why" or more precisely the goal of the technique used by the threat actor.
        - A tactic could be have the goal of: 
            - Gaining vital information of a target (*Reconnaissance*).
            - Gaining access to the target system (*Initial Access*).
            - Achieving privilege escalation in the target system (*Privilege Escalation*).
    - Techniques
        - Represents the "how" to achieve the chosen tactic.  
        - A technique could be:
            - Gathering target information by analyzing the target network (*Gather Victim Identity Information*).
            - Gaining initial access to the target system by exploiting a weakness in the target system (*Exploit Public-Facing Application*).
            - Achieving privilege escalation in the target system by abusing the systems elevation control mechanism (*Abuse Elevation Control Mechanism*).
        - Techniques can also be explained by sub-techniques, which describe the technique more precisely.
            - For instance, gathering target information by analyzing the target networks IP-addresses (*Gather Victim Network Information: IP Addresses*).
    - Procedures
        - Describes how a threat actor uses techniques and sub-techniques to achieve a chosen tactic.
        - Procedures could be:
            - A threat actor gathering target information by analyzing the target networks IP-addresses by using nmap (a network discovery and security auditing tool).
            - A threat actor gaining initial access to the target system by exploiting a weakness in the target system by using a SQL-injection.
            - A threat actor achieving privilege escalation in the target system, abusing the systems elevation control mechanism by using Cobalt Strike.
- So how do tactics, techniques, and procedures represent themselves in the ATT&CK Enterprise Matrix?
    - Tactics are the column headers in the matrix (left to right)
    - Techniques are boxes under the header, you also could call them rows.
        - Sub-techniques (if available) are listed within the techniques.
    - Procedures are listed within a given technique and/or sub-technique.
    - In the previous examples, precise ATT&CK Enterprise Matrix tactics, techniques and sub-techniques are within round brackets written in *italic*.   
- ATT&CK Enterprise Matrix consists of tactics in a logical and chronological order a threat actor would proceed in, from reconnaissance to impact.

### a

The Cyber Kill Chain and ATT&CK Enterprise Matrix serve different purposes and in my view aren't comparable, even though they can complement each other and work side by side.
The Cyber Kill Chain provides a model for analyzing, identifying and mitigating intrusions and developing a more robust approach on cyber security. 
The ATT&CK Enterprise Matrix on the other hand, consists of documented incidents and guidelines on how mitigate them.
These two can coexists, ATT&CK Enterprise Matrix providing knowledge of tactics and techniques used to battle the APT:s in different phases of the Cyber Kill Chain, while the new aquired knowledge of incidents from the Cyber Kill Chain can be added to the ATT&CK Enterprise Matrix.


>Who do you think could benefit from these models?

I don't know if this is a tough question, since I had to contemplate on it for a while. Generally I would divide the beneficial parties in two, one for The Cyber Kill Chain and another for the ATT&CK Enterprise Matrix.
The Cyber Kill Chain benefits most people and organizations working in or with cyber security, since it deals more on on active and adaptive defense measures.
I see the ATT&CK Enterprise Matrix benefiting system and network administrators, since it provides knowledge on how to defend against intrusion tactics and techniques that are already documented.
There is also the notion that both parties would benefit from both models, since I see these two models working side by side strengthening each other as stated before.
There is also a third group that would benefit from these models, maybe a bit indirectly with out dwelling too much on the technical side and these are the decision makers, CEO:S, COO:s and CTO:s. Reviewing these should raise the question "are we doing enough", "should we do more", "Are we already doing something".

### b

I decided to search Krebs on security for an incident to investigate and I found an interesting case involving Github users. 
The incident report can be read at the Krebs on security website: [This Windows PowerShell Phish Has Scary Potential](https://krebsonsecurity.com/2024/09/this-windows-powershell-phish-has-scary-potential/)

### c

#### Foreword
It has been a few years since I last used Virtualbox, I have had little need to run full virtualization, since containers (Docker for Linux, Jails for BSD) fulfill my current needs. Last time I used a full virtualization platform  was most likely virt-manager (libvirt), since I managed the virtual machines using cockpit, although backup automation was done using virsh on the command line.

#### System enviroment
- Computer
    - CPU: AMD 7700 (Clocked @ 5.45 GHz)
    - Motherboard: ASUS B650M-plus-wifi
    - Memory: 32 GB DDR5 6000MHz
    - Graphics card: AMD RX 7900XTX
    - Storage: 1TB Western digital black SN850X (m.2, nvme)

- Operating system: EndeavourOS (Arch linux)
    - Kernel version: 6.11.5-arch1-1
    - Desktop: KDE plasma 6 (Wayland)
    - Package manager: pacman

#### Virtualbox Installation

I usually review the Arch linux wiki for information, just in case. [Virtualbox Arch wiki page](https://wiki.archlinux.org/title/VirtualBox).

Update the system. Review upgrades if there is any and aprove if deemed acceptable (This step is not mentioned again, but it's always a good practice to even glance what you are about to update)
- *sudo pacman -Syu*

Reboot system if required.

Install Virtualbox and the required dependencies if there is any. Note that the Arch linux offers three (3) different host module packages, consult the Arch wiki page for the right one for you. In my case it is "virtualbox-host-modules-arch", since I only use the LTS kernel as a backup option.
- *sudo pacman -S virtualbox*

Reboot system or manually load the Virtualbox kernel module.
- To load manually *sudo modprobe vboxdrv*

Other considerations regarding the installation on Arch linux.
- To enable USB access from the host machine, add user to the vboxusers group. I am not planning on using this now.
    - *sudo usermod -a -G vboxusers username*
- **Extension pack**, this is a new feature for me. The Oracle VM VirtualBox Extension Pack should provide additional features, but I don't think we need them now.
Start the regular Virtualbox GUI.
    - *VirtualBox*

![Arch linux Virtualbox installation](/img/virtual_box_install.png)

#### Virtual machine installation

Download the installation ISO image from the [Debian home page](https://www.debian.org/)
Create a new virtual machine in Virtualbox.

- Click the *New* icon
    - Set VM name
    - Set VM location, the default location (user home folder) is OK, since we probably don't have access rights outside our home folder (no super user privileges used)
- Select ISO image
    - Check "Skip unattended Installation", since we want to do things manually. 
     Click *Next*
- Select memory amount and processor count, let's stick with the defaults.
    - Click *Next*
- Create a virtual hard disc for the Virtual machine. 
    - 20 GB is OK for this test.
    - No pre-allocation needed, pre-allocation reserves the entire disc size, instead of used space.
    - Click *Next*
-  Review and click finish if everything is in order.

| New VM                                | Memory                                | Virtual hard disc                             | Review                                |
| ------------------------------------- | -----------------------------------   | --------------------------------------------- | ------------------------------------- |
| ![](/img/Virtualbox_vm_install_1.png) | ![](/img/Virtualbox_vm_install_2.png) | ![](/img/Virtualbox_vm_install_3.png)         | ![](/img/Virtualbox_vm_install_4.png) |

Start the VM by clicking *Start*

![](/img/Virtualbox_vm_install_5.png)

And at this stage I remembered that I have disabled the virtualization support on the motherboard. Don't enable it if you are not using it they say...
Let's reboot the system and make the necessary changes in the UEFI and try again.

![](/img/Virtualbox_vm_install_6.png)

Everything seems to be in order now.

![](/img/Virtualbox_vm_install_7.png)

#### Debian Virtual machine installation

Let's select the graphical installation.
- Select installation language, English will do in our case.
- Select location, Finland can be found under the "Other" selection, but let's just say we are in the US.
- Select keyboard layout. NOTE! US usually uses an ANSI keyboard layout, which is ok but I prefer ISO (used in Finland for example).
- Select hostname for the new VM. "vbox" is OK in this case, but you should select a unique hostname for each computer for the sake of convenience.

| Language                              | Location                              | Kayboard layout                               | Hostname                              |
| ------------------------------------- | -----------------------------------   | --------------------------------------------- | ------------------------------------- |
| ![](/img/Virtualbox_vm_install_8.png) | ![](/img/Virtualbox_vm_install_9.png) | ![](/img/Virtualbox_vm_install_10.png)        | ![](/img/Virtualbox_vm_install_11.png)|

- No domain name needed, but let's use something that should be safe => *.local*
- Set password for the root (system administrator) account.
    - Let's use something short this time, since this is a test installation. Otherwise you should use a good password, preferably randomly generated with with letters (small and large), numbers and special characters.
- Create a user account, let's use the same username as the host system. Note, usually the system asks your'e name and generates a username based on that. I like to skip this and define my username manually.
- Add a password fot the new user account.
- Configure system clock, select time zone. Note, the VM thinks we are in the US..

| Domain name                               | Root password                                 | User account                                  | User account password                     | System Clock                              |
| ----------------------------------------- | --------------------------------------------- | --------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![](/img/Virtualbox_vm_install_12.png)    | ![](/img/Virtualbox_vm_install_13.png)        | ![](/img/Virtualbox_vm_install_14.png)        | ![](/img/Virtualbox_vm_install_15.png)    | ![](/img/Virtualbox_vm_install_16.png)    | 


- Select how the disc is partitioned. Let's stick to the default in this case.
- Select disc used (only one available as a default)
- Confirm choices and continue

| Disc partitioning                         | Disc selection                                | Partitioning options                          |  Partitioning overview                    | Partitioning confirmation                 |
| ----------------------------------------- | --------------------------------------------- | --------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![](/img/Virtualbox_vm_install_17.png)    | ![](/img/Virtualbox_vm_install_18.png)        | ![](/img/Virtualbox_vm_install_19.png)        | ![](/img/Virtualbox_vm_install_20.png)    | ![](/img/Virtualbox_vm_install_21.png)    | 

- Wait for the installation to complete

![](/img/Virtualbox_vm_install_22.png)

- No need to scan for another installation media for the package manager.
- Select location and mirror to be used with the package manager 
- No need for a proxy
- Wait for the package manager configuration.

| Additional media                          | Mirror country                                | Mirror address                                |  HTTP proxy                               | Configuration                             |
| ----------------------------------------- | --------------------------------------------- | --------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![](/img/Virtualbox_vm_install_23.png)    | ![](/img/Virtualbox_vm_install_24.png)        | ![](/img/Virtualbox_vm_install_25.png)        | ![](/img/Virtualbox_vm_install_26.png)    | ![](/img/Virtualbox_vm_install_27.png)    | 


- Let's not participate in this for now.

![](/img/Virtualbox_vm_install_28.png)

- Select desktop environment and system utilities, KDE plasma is usually a good choice, xfce is another good choice.
- Wait for the installation to finish
- Install CRUB boot loader, select the VM system disc for this.
- Wait for the installation to finnish and click *Continue* to reboot the new VM.

| Desktop environment & utilities           | Software installation                         | GRUB installation                             | GRUB installation location                | Finish installation                       |
| ----------------------------------------- | --------------------------------------------- | --------------------------------------------- | ----------------------------------------- | ----------------------------------------- |
| ![](/img/Virtualbox_vm_install_29.png)    | ![](/img/Virtualbox_vm_install_30.png)        | ![](/img/Virtualbox_vm_install_31.png)        | ![](/img/Virtualbox_vm_install_32.png)    | ![](/img/Virtualbox_vm_install_33.png)    | 

- Once the VM reboots, select the default choice in GRUB (or just wait) and you can log in to you're new VM

| Login screen                          | New VM desktop                          |
| ------------------------------------- | --------------------------------------- |
| ![](/img/Virtualbox_vm_install_34.png) | ![](/img/Virtualbox_vm_install_35.png) | 

### d

### e

### f