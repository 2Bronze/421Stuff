const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const mongoose = require('mongoose');
const server = require('./server');

const port = process.env.PORT || 8080;

const app = express()
            .use(cors())
            .use(bodyParser.json())
            .use(server());

mongoose.connect('mongodb://localhost:27017/main')
  .then(() => {
    console.log("Connected to database");
    app.listen(port, () => {
      console.log(`Express server listening on port ${port}`);
    });
  });
