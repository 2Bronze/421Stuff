const express = require('express');
const users = require('./user');
const fs = require('fs');

function createRouter() {
  const router = express.Router();

  router.get('/profile/:username', users.getUserDocument,
             (req, res, next) => {
               const user = req.userDocument;
               res.status(200).json({
                 username: user.username,
               });
             }
  );

  router.put('/profile', async (req, res, next) => {
    const exists = await users.User.exists({username: req.user.username});
    if (exists) {
      return res.status(400).json({status: 'user-exists'});
    }
    await users.User.create({username: req.body.username});
    res.status(200).json({status: 'ok'});
  });

  return router;
}

module.exports = createRouter;
