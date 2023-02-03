const { evolve } = require('../scripts/evolve');

evolve('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
