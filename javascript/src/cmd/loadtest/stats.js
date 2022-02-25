const hdr = require('hdr-histogram-js');
const moment = require('moment');

const log = require('../../logger');

class Stats {
  constructor() {
    this.hist = hdr.build({
      lowestDiscernibleValue: 1,
      highestTrackableValue: 1000,
      numberOfSignificantValueDigits: 2,
    });
    this.ts = moment();
    this.total = 0;
  }

  record(start) {
    let duration = moment().diff(start);
    this.hist.recordValue(duration);
  }

  print(prefix) {
    if (moment().diff(this.ts) < 1000) {
      return;
    }

    // from the report print only the relevant stats
    const output = this.hist
      .outputPercentileDistribution()
      .split('\n')
      .filter(line => line.startsWith('#['))
      .join('\n');

    log.info(`${prefix}\n${output}`);
  }
}

module.exports = { Stats };
