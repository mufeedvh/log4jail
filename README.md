# `log4jail` 🛡️

A fast firewall reverse proxy with TLS (HTTPS) and swarm support for preventing Log4J (Log4Shell aka CVE-2021-44228) attacks.

## 📖 Table of Contents

- [Introduction](#%E2%84%B9%EF%B8%8F-introduction)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Proof of Concept](#proof-of-concept)
- [For Pentesters](#for-pentesters)

## ℹ️ Introduction

**log4jail** is a quick and dirty solution to block Log4Shell exploit attempts by acting as a reverse proxy scanning complete request body including all headers and request parameters for log4shell payloads.

💡 You can run multiple instances at once by mapping them by port and target in the configuration file.

> The firewall follows the [mitigation strategies from Google Cloud](https://cloud.google.com/blog/products/identity-security/recommendations-for-apache-log4j2-vulnerability).

## Installation

    $ cargo install --git https://github.com/mufeedvh/log4jail.git
    
[Install Rust/Cargo](https://rust-lang.org/tools/install)

## Build From Source

**Prerequisites:**

* [Git](https://git-scm.org/downloads)
* [Rust](https://rust-lang.org/tools/install)
* Cargo (Automatically installed when installing Rust)
* A C linker (Only for Linux, generally comes pre-installed)

```
$ git clone https://github.com/mufeedvh/log4jail.git
$ cd log4jail/
$ cargo build --release
```

The first command clones this repository into your local machine and the last two commands enters the directory and builds the source in release mode.

## Quick Start

Just run the command and there will be a boilerplate configuration file in the same working directory called `log4jail.json`.

    $ log4jail
    
The configuration file looks like this:    
    
```json
{
  "host": "127.0.0.1",
  "enable_https": false,
  "tls_cert_path": "cert.pem",
  "tls_key_path": "key.rsa",
  "reject_response_status": 403,
  "reject_response_body": "This request has been blocked.",
  "proxy_mapping": {
    "1337": "http://127.0.0.1:8080/",
    "6969": "http://127.0.0.1:9000/"
  }
}
```

Configure the values for your environment and run the command again to spin up the proxy!

## Proof of Concept

`log4jail` blocks all attack vectors from [`log4j-scan`](https://github.com/fullhunt/log4j-scan) with WAF Bypass mode enabled:

    $ python3 log4j-scan.py -u http://127.0.0.1:1337/ --run-all-tests --waf-bypass
    
![Screenshot from 2021-12-15 01-34-30](https://user-images.githubusercontent.com/26198477/146071752-2e105a65-aec5-4f67-81ea-8dd9caeff3a6.png)

## For Pentesters

If you have found any working Log4Shell payloads that can bypass this firewall, please [open an issue](https://github.com/mufeedvh/log4jail/issues/new).

Just run `log4jail` on any port and include the payload in the HTTP request for fuzz testing.

---
