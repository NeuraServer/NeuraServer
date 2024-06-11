const express = require('express');
const bodyParser = require('body-parser');
const axios = require('axios');

const app = express();
const port = 3000;

app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// Endpoint to fetch data from a specified IP
app.get('/fetch-data', async (req, res) => {
    const ip = req.query.ip;
    if (!ip) {
        return res.status(400).send('IP address is required');
    }

    try {
        const response = await axios.get(`http://${ip}`);
        res.send(response.data);
    } catch (error) {
        res.status(500).send('Failed to fetch data from the specified IP address');
    }
});

// Endpoint to handle incoming data and respond
app.post('/process-data', (req, res) => {
    const data = req.body.data;
    if (!data) {
        return res.status(400).send('Data is required');
    }

    // Process the data (this can be customized as needed)
    const processedData = data.toUpperCase();  // Example processing
    res.send({ processedData });
});

// Endpoint to check server status
app.get('/status', (req, res) => {
    res.send('NeuraServer Azurite is running');
});

app.listen(port, () => {
    console.log(`NeuraServer Azurite is running at http://localhost:${port}`);
});
