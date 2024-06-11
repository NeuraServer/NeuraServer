const axios = require('axios');

const SERVER_IP = '127.0.0.1';
const SERVER_PORT = '5500';
const SERVER_URL = `http://${SERVER_IP}:${SERVER_PORT}`;

async function sendData(endpoint, data) {
  try {
    const response = await axios.post(`${SERVER_URL}/${endpoint}`, data);
    console.log(`Response from ${endpoint}:`, response.data);
  } catch (error) {
    console.error(`Error sending data to ${endpoint}:`, error.message);
  }
}

async function fetchData(endpoint) {
  try {
    const response = await axios.get(`${SERVER_URL}/${endpoint}`);
    console.log(`Data from ${endpoint}:`, response.data);
  } catch (error) {
    console.error(`Error fetching data from ${endpoint}:`, error.message);
  }
}

// Example usage
(async () => {
  const exampleData = {
    key: 'value',
    anotherKey: 'anotherValue'
  };

  // Send data to the server
  await sendData('api/send', exampleData);

  // Fetch data from the server
  await fetchData('api/data');
})();
