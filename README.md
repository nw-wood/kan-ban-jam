### My kan-ban-jam Project

Kan-Ban-Jam Project

What's This?

    A lightweight Kanban board that lets you manage tasks in real-time.
    Built as a learning project to understand modern web development and explore how real-time applications work.
    Think of it as a digital version of those sticky-note boards you see in tech companies.

Why Build Another Kanban Board?

Two main reasons:

    To learn WebSocket implementation and real-time features
    To understand modern web development without framework complexity

Key Features

    Real-Time Updates: Changes from any client are immediately visible to all connected users
    Asynchronous Backend: Server handles operations asynchronously for better performance
    Lightweight Design: No front end depenencies, focused on core functionality

![kan-ban-jam](/screenshot.png?raw=true "kan-ban-jam")

depends on:
```
tokio = { version = "1.40.0", features = ["full"] }
serde = { version = "1.0.210", features = ["derive"] }
serde_json = "1.0.128"
env_home = "0.1.0"
regex = "1.11.0"
warp = "0.3.7"
futures-util = "0.3.30"
```
