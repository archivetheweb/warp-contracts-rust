const { archivesByURL: archivesByURL } = require('../scripts/read');

archivesByURL('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
