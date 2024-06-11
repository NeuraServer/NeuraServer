![image (46)](https://github.com/NeuraServer/NeuraServer/assets/140754373/1746345a-d4a8-4521-8895-95ccf9941f9e)

# Neuraserver Azurite
NeuraServer Azurite is a service provided by NeuraServer for hosting servers online, this can be accomplished via
a node.js server, JS with IP server, and other.
## Making the server
First, you need to install Node.JS and Express, you can do it on the official website:
[NodeJS Download](https://nodejs.org/en)
Express Download:
```bash
npm install express body-parser axios
```
Open up the terminal, and run this:
```bash
mkdir neuraserver-azurite
cd neuraserver-azurite
```
Then, copy the files provided in /src in this same directory.
After that, you should see the server.js file, where you can set everything up.
## Running the server/instance
Open up your terminal, and run this:
```bash
node server.js
```
Then everything should work.
## Using cURL for IP
To run the server with an IP, you will need to install cURL:
[cURL Download](https://curl.se/download.html)
First, specify your IP, or other adress/host that you need. The default is your current machine (localhost).
```bash
curl http://localhost:3000/fetch-data?ip=127.0.0.1:5500
```
Process incoming data:
```bash
curl -X POST -H "Content-Type: application/json" -d '{"data": "Hello World"}' http://localhost:3000/process-data
```
Check status:
```bash
curl http://localhost:3000/status
```
Now, everything should be set up!
## Interacting with NeuraServer
You can use Axios to interact with it, heres how to download it:
```bash
npm install axios
```
Now, there will be a script in src/ called shipping.js, there, you can fully interact with the NeuraServer API and servers.
