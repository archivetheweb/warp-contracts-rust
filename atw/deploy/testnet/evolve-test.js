const { evolve } = require('../scripts/evolve');

evolve('arweave.net', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
