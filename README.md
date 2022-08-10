# Rust_HTTP_Method_Enumerator
Identify potentially exploitable HTTP methods using Rust

<img src="/images/output.png" alt="Alt text" title="Optional title">

The Rust HTTP Method Enumerator can take a number of options. 

## Required Arguments:

#### Scan a single URL for allowed methods:
--url https://example.com

#### Scan a wordlist of URLs for allowed methods:
--wordlist list-of-urls.txt


## Optional Arguments:

#### Send a cookie with each request:
--cookie "paste value here"

#### Send a body with each request
--body "paste text here"


