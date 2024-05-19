# wlrs

Authenticated web-based whitelister for a minecraft server.


## dotenv (.env)
```bash
# server
SERVER_ADDR = "<ADDR>:<PORT>"
SERVER_PASS = "<PASSWORD>"
RCON_PASS = "<PASSWORD>"

# client
WEBSOCKET_ADDR = "wss://<ADDR>/<PATH>"
```
- This is the configuration for the local `.env` file.

---

### Authentication
It's very basic. The provided `SERVER_PASS` is hashed with a randomly generated salt upon compilation. A user provides a text-based string as the password, which is then sent to the server via secure websocket. The password is then hashed with the existing salt and compared with the existing hash. If they match, the password is correct.

---

### Todo
- Implement rate-limiting by IP-address $\to$ prevent basic brute-force attempts