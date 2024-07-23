# Api-Gateway

### This is my personal practice project to advance my rust expertise using axum and diesel.
---

### Summary
- I wrote a simple web-server in rust that may act as an api gateway, managing users' auth_tokens and performing other tasks such as logging
- I learned a lot about Rust project structure, syntax and best-practices
---

### Original Objectives
Most of them have been implemented.
-   can manage users and their api keys via api
-   has a few sample endpoints that fulfill no real purpose
-   when calling upon those endpoints, the following happens
    -   authenticates client via api token
        -   (maybe add authorization for different endpoints in the future)
    -   Log incoming requests and responses for auditing and debugging purposes (in db? in logfile? different drivers available?)
    -   tracks users requests statistics for the different endpoints
    -   Monitor server performance metrics (e.g., response time, error rate)
    -   Define standardized error responses for various types of errors
        -   Implement error middleware to handle exceptions and return appropriate responses


