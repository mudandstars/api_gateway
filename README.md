# Api-Gateway

### This is my personal practice project to advance my rust expertise using axum and diesel.

### (Possible) Goals

-   can manage users via api
-   has a few sample endpoints that fulfill no real purpose
-   when calling upon those endpoints, the following happens
    -   authenticates client via api token
        -   (maybe add authorization for different endpoints in the future)
    -   Log incoming requests and responses for auditing and debugging purposes
    -   tracks users requests statistics for the different endpoints
    -   Monitor server performance metrics (e.g., response time, error rate)
    -   Define standardized error responses for various types of errors
        -   Implement error middleware to handle exceptions and return appropriate responses
-   Generate API documentation (e.g., using OpenAPI/Swagger) for developers
