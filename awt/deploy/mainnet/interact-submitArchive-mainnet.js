const { submitArchive: submitArchive } = require('../scripts/interact');

submitArchive('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
