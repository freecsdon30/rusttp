### Hello everyone! This project is currently under development. Please comeback after a while..!!


### Phase 1: The Foundation
1. CLI Interface: Implement argument parsing for --host and --port using clap.
2. TCP Socket Binding: Initialize TcpListener based on CLI inputs.
3. Request Parsing: Read raw stream data and extract Method, URL path, and HTTP version.
4. Basic Response: Return a standard 200 OK status line with a "Hello World" body.
5. Robust Error Handling: Replace all .unwrap() calls with custom Result types to prevent crashes.

### Phase 2: Core Web Features
6. Static File Serving: Serve files from a local directory based on the URL path.
7. Directory Traversal Protection: Sanitize paths to prevent access to files outside the target directory.
8. Request Body Handling: Read and parse data from POST and PUT requests.
9. Logging & Observability: Integrate tracing or log crates to track incoming requests and system errors.
10. Connection Closure: Properly manage stream lifecycle and EOF (End of File) states.

### Phase 3: Performance & Concurrency
11. Multi-threaded Handling: Move from a single-threaded loop to std::thread::spawn.
12. Thread Pool implementation: Implement a custom thread pool or integrate rayon/threadpool to manage resources.
13. Request Timeouts: Set read/write deadlines to prevent slow-loris attacks or hung connections.
14. Persistent Connections: Implement Keep-Alive logic to reuse TCP connections for multiple HTTP requests.
15. Graceful Shutdown: Listen for SIGINT (Ctrl+C) to finish active requests before closing the process.

### Phase 4: Production Grade Enhancements
16. Compression Engine: Support Accept-Encoding and implement Gzip/Brotli compression.
17. Security Headers: Automatically inject HSTS, X-Content-Type-Options, and CSP headers.
18. TLS Support (HTTPS): Integrate rustls to handle encrypted traffic.
19. Health Check Endpoint: Provide a /health route for monitoring and load balancers.
20. Middleware Support: Create a system to easily plug in functionality like authentication or custom headers.

