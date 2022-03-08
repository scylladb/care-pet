const winston = require('winston');

const logger = winston.createLogger({
  level: process.env.CARE_PET_LOG_LEVEL || 'info',
  transports: [
    new winston.transports.Console({
      format: winston.format.simple(),
    }),
  ],
});

module.exports = logger;
