## MILESTONE 2 PLAN: [Project Name]

**Team:** SMH Domains  
**Track:** [ ] SHIP-A-TON [✅] IDEA-TON  
**Date:** 2025-11-16

---

## 📍 WHERE WE WERE UP TO NOW

The project you see here is made up of 4 core parts:
- A smart contract to act as the registrar
- A DNS server setup to be the basis of domain resolution
- A registration website for clients to use
- A daemon script to sync blockchain state with the DNS server's database (domain & zone tables)

Of the parts above, parts 1 & 2 were already created at LatinHack2025, and what we have here is the website & the daemon script.

## 📍 WHERE WE ARE NOW

**What we built/validated this weekend:**
- The user can register the domains add/updat DNS zones & transfer domain ownership
- We have a script that can listen to the contracts events

**What's working:**
- The DNS servers are up and running (Both authoritative and recursive ones, the authoritative server resolves domains from a MySQL database)
- The smart contract is working as expected

**What still needs work:**
- The script which listens to contract events to later add domain names and update zones isn't still fully functional
- There are still UI/UX improvements needed for the website

**Blockers or hurdles we hit:**
- Ink! ecosystem being so young, accessing documentations and reliable LLMs to create the smart contract was so hard

---

## 🚀 WHAT WE'LL SHIP IN 30 DAYS

**Our MVP will do this:**
SMH Domains will be a reliable domain name registry for blockchain enabled domains, uncersorable, unstoppable and respecting user's privacy

### Features We'll Build (3-5 max)

**Week 1:**
- Feature: A fully functional daemon script
- Why it matters: without it the project fails
- Who builds it: Amirreza

**Week 2:**
- Feature: Decentralized SSL Authority
- Why it matters: Creating a directory of SSL authorities and marketing it to invite various entities to decentralize the way SSL certificates are issued
- Who builds it: Amirreza & Eni

**Week 2-3:**
- Feature: DEB/RPM Package for self hosting an SMH clone
- Why it matters: The packages can let everyone set up his own Polkadot enabled DNS server easily and make SMH DNS servers much broader, 
- Who builds it: Amirreza

**Week 3-4:**
- Feature: Marketing for SSL authorities and self hosting SMH DNS servers
- Why it matters: If there are no people to hear our voice, the voice will die.
- Who builds it: Eni

### Team Breakdown (if applicable)

**Amirreza - BackEnd & Blockchain Developer** | 10 hrs/week

**Eni Korinkova - Digital Marketer & Blockchain Enthusiast** | 10 hrs/week

### Mentoring & Expertise We Need

**Areas where we need support:**
- The code we wrote needs improvement, it'd be great to have software developer mentors who help us write better code, specially Javascript codes to interact with the blockchain
- Setting up DNS server in users' devices is hard for some of them and indeed doesn't leave a good UX, being able to negotiate with corporations behind big Operating Systems can be a huge thing for the project

**Specific expertise we're looking for:**
- Blockchain developer fluent in Javascript

---

## 🎯 WHAT HAPPENS AFTER

**When M2 is done, we plan to...** 
- We plan to found a non-profit in a neutral country (i.e. Switzerland or Singapore etc.) to make our dream of having self custodial domains come true