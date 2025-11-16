# 🌐 SMH Domains

**SMH Domains** is a **self-custodial domain name service** built on top of the **Polkadot blockchain** (currently deployed on the **Passet Hub parachain**).  
Domains are registered directly **on-chain**, and users can **add or update DNS zones** securely without relying on centralized registrars.

---

## 🚀 Demo

🔗 **Live Demo:** [https://smh.elamir.ch](https://smh.elamir.ch)

Try resolving `.smh` domains through our live DNS instance at **`91.107.151.44`**.

Example:

```bash
dig @91.107.151.44 latinhack.smh
````

---

## 🧠 How It Works

* Every `.smh` domain is **registered on the blockchain** using SMH smart contracts.
* Domain metadata (DNS zones) are stored and synchronized by the `smh_dns_plugin` script.
* The **PowerDNS authoritative server** hosts `.smh` zones.
* A **recursive PowerDNS server** forwards `.smh` queries to the authoritative server, while handling normal DNS queries normally.

---

## ⚙️ Setup Guide

### 1️⃣ Install PowerDNS Authoritative Server

#### Ubuntu / Debian:

```bash
sudo apt update
sudo apt install pdns-server pdns-backend-sqlite3
```

You can use SQLite, MySQL, or PostgreSQL as a backend.
For simplicity, we'll use **SQLite** in this guide.

#### Configure the backend:

Edit `/etc/powerdns/pdns.conf`:

```ini
launch=gsqlite3
gsqlite3-database=/var/lib/powerdns/pdns.sqlite3
```

#### Initialize the database:

```bash
sudo pdnsutil create-zone example.smh ns1.smh.local
sudo systemctl restart pdns
```

---

### 2️⃣ Install PowerDNS Recursor

This will handle all queries and forward `.smh` domains to your authoritative server.

```bash
sudo apt install pdns-recursor
```

Edit `/etc/powerdns/recursor.conf`:

```ini
forward-zones=smh=91.107.151.44
```

Restart the service:

```bash
sudo systemctl restart pdns-recursor
```

Test it:

```bash
dig @127.0.0.1 example.smh
```

If everything is configured properly, you should see your `.smh` domain resolving via the blockchain-powered authoritative server.

---

## 🔁 Syncing Domains from the Blockchain

The `smh_dns_plugin` script continuously reads `.smh` domain updates from the **Passet Hub blockchain** and updates the PowerDNS authoritative server database.

### Install the Plugin

Clone the repo:

```bash
git clone https://github.com/elamirch/smh-domains.git
cd smh-domains
```

Set executable permissions:

```bash
chmod +x smh_dns_plugin.js
```

### Add to Cron

Edit your crontab:

```bash
crontab -e
```

Add this line to sync every 5 minutes:

```bash
*/5 * * * * /path/to/smh-domains/smh_dns_plugin.js >> /var/log/smh_sync.log 2>&1
```

This ensures your DNS zones stay up-to-date with the latest blockchain state.

---

## 🧩 Architecture Overview

```
┌────────────────────────────┐
│   Polkadot / Passet Hub    │
│     (SMH Registry)         │
└────────────┬───────────────┘
             │
             ▼
     smh_dns_plugin (cron)
             │
             ▼
┌────────────────────────────┐
│  PowerDNS Authoritative    │
│  - Hosts .smh zones        │
└────────────┬───────────────┘
             │
             ▼
┌────────────────────────────┐
│  PowerDNS Recursor         │
│  - Forwards .smh → Auth    │
│  - Resolves others normally│
└────────────────────────────┘
```

---

## Related repos:

- [smh-dns-plugin](https://github.com/elamirch/smh-dns-plugin)  
- [smh-domains-website](https://github.com/elamirch/smh-domains-website)


## 🧪 Testing

Use the sample DNS server to test `.smh` domain resolution:

```bash
dig @91.107.151.44 example.smh
```

Or via your local recursor setup:

```bash
dig @127.0.0.1 example.smh
```

---

## 📜 License

MPL 2.0 License

---

## 🤝 Contributing

Contributions are welcome!
Please open an issue or submit a PR if you’d like to improve SMH Domains.

---
