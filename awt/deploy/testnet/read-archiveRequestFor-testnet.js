const { readArchiveRequestFor: readArchiveRequestFor } = require('../scripts/read-archiveRequestFor');

readArchiveRequestFor(
  'testnet.redstone.tools',
  443,
  'https',
  'testnet',
  'deploy/testnet/wallet_testnet.json'
).finally();
