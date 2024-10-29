const express = require('express');
const cors = require('cors');
require('dotenv').config();

const app = express();
app.use(cors());
app.use(express.json());

const PORT = process.env.PORT || 3000;

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});

app.post('/dataset', (req, res) => {
    res.json({ message: 'Dataset registered successfully' });
});

app.post('/usecase', (req, res) => {
    res.json({ message: 'Use case registered successfully' });
});

