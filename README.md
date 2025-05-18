# DispatchFlow

Right now, emergency coordination is slow and fragmented. Teams rely on outdated systems that delay response time, and in critical moments, that delay can cost lives.

I built DispatchFlow, a lightweight, web-based platform where teams can instantly join, update status, and see each other live. No setup. No lag. Just fast, reliable coordination.

I chose Rust for the backend, it’s fast, memory-safe, and perfect for real-time systems. The frontend is hand-coded in HTML and CSS, and the map is powered by OpenStreetMap and LeafletJS, optimized for mobile. Even the SQLite integration was manually written to keep things lightweight and efficient.

I built everything myself: backend, frontend, user auth, live geolocation, and team status tracking. Yeah, even the SQ and I hate SQL.

But here’s the thing: I didn’t use AI. At all. No ChatGPT wrappers, no vibe-coded frontends in Cursor, no autocompleted boilerplate I didn’t understand.

In high-stakes systems like First Responders’, lives are on the line. You don’t want code written by a chatbot, wrapped in a black box. One silent bug, one crash, and it’s not just downtime. It’s headlines. It's a tragedy.

That’s why I made the hard choice: hand-code everything. Line by line. It took longer, but I gained control, confidence, and complete understanding of my system. I’d rather debug a real bug I wrote than untangle thousands of lines of AI guesswork.

When reliability matters, fast AI-generated demos don’t cut it. DispatchFlow is something real, something I trust to work under pressure.

This could be used in emergency response, field teams, search-and-rescue, anywhere that demands instant team setup and real-time coordination.
