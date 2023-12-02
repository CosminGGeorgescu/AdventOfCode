# Rusty Retriever

Santa's little helper for retrieving each day's puzzle input.

It asks for the query year and day, where $year >= 2015$ and $day \in [1..=25]$. Of course the day for the current year has to be $<=$ than the current day.

It only fashionably works with my suggested folder structure: 

```sh
    .
    ├─ tools
    ├─ year<n>: Workspace Member Library
        ├─ input
           ├─ 1.txt
           ├─ ...
           └─ 25.txt
        ...
    ├─ COOKIES.txt
    └─ src: Workspace
        lib.rs
```

and uses a `COOKIES.txt` Netscape file for storing cookies. The only reasonable cookie you would want to have is the session_id with the following format

```sh
    DOMAIN_NAME         SUBDOMAINS  PATH   HTTPS_ONLY   EXPIRY_DATE   NAME     VALUE
    .adventodcode.com   FALSE       /      TRUE         <number>      session  <string>
```
