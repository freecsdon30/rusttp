### Hello everyone! This project is currently under development. Please comeback after a while..!!


### Phase 1: The Foundation
- [x] CLI Interface: Implement argument parsing for --host and --port using clap. 
- [x] CP Socket Binding: Initialize TcpListener based on CLI inputs.
- [x]  Request Parsing: Read raw stream data and extract Method, URL path, and HTTP version.
- [x]  Basic Response: Return a standard 200 OK status line with a "Hello World" body.
- [x]  Robust Error Handling: Replace all .unwrap() calls with custom Result types to prevent crashes.

### Phase 2: Core Web Features
- [ ]  Static File Serving: Serve files from a local directory based on the URL path.
- [ ]  Directory Traversal Protection: Sanitize paths to prevent access to files outside the target directory.
- [ ]  Request Body Handling: Read and parse data from POST and PUT requests.
- [ ]  Logging & Observability: Integrate tracing or log crates to track incoming requests and system errors.
- [ ]  Connection Closure: Properly manage stream lifecycle and EOF (End of File) states.

### Phase 3: Performance & Concurrency
- [ ]  Multi-threaded Handling: Move from a single-threaded loop to std::thread::spawn.
- [ ]  Thread Pool implementation: Implement a custom thread pool or integrate rayon/threadpool to manage resources.
- [ ]  Request Timeouts: Set read/write deadlines to prevent slow-loris attacks or hung connections.
- [ ]  Persistent Connections: Implement Keep-Alive logic to reuse TCP connections for multiple HTTP requests.
- [ ]  Graceful Shutdown: Listen for SIGINT (Ctrl+C) to finish active requests before closing the process.

### Phase 4: Production Grade Enhancements
- [ ]  Compression Engine: Support Accept-Encoding and implement Gzip/Brotli compression.
- [ ]  Security Headers: Automatically inject HSTS, X-Content-Type-Options, and CSP headers.
- [ ]  TLS Support (HTTPS): Integrate rustls to handle encrypted traffic.
- [ ]  Health Check Endpoint: Provide a /health route for monitoring and load balancers.
- [ ]  Middleware Support: Create a system to easily plug in functionality like authentication or custom headers.

### Functionalities to add

- [x]  Acceptor/Listener  - function to bind to a socket
- [ ]  Non blocking event loop - Reactor pattern 
- [ ]  TLS termination - wrap raw streams in encryption layer 
- [x]  Request/Response structs - type that holds methods, URIs and status codes
- [ ]  HeaderMap: a header map that is efficient and supports lazy parsing 
- [ ]  Streaming body: loading stream of bytes rather than loading whole body at once 
- [ ]  Service Trait: defines a function taking request and returning future of response.
- [ ]  Router: a function that matches URI to its specific handler function 
- [ ]  middleware: functions that wrap handlers for specific purposes
- [ ] Keep-Alive management: Functions to manage persistent connections, allowing multiple requests over a single TCP connection.
- [ ] Payload decompression: Middleware to automatically handle gzip or br encodings to reduce bandwidth.
- [ ] Error mapping: for status code and messages 
- [ ] Graceful Shutdown: Logic to stop accepting new connections while finishing active requests before the process exits.
- [ ] Resource Throttling: logic to control concurrency
