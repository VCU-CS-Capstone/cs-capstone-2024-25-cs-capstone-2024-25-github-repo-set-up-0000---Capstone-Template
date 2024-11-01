const mockData = require('../utils/mockData');

const getAllDatasets = async (req, res) => {
    try {
        res.json(mockData.datasets);
    } catch (error) {
        res.status(500).json({ message: "Error fetching datasets" });
    }
};

module.exports = {
    getAllDatasets
};