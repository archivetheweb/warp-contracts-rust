const { readArchiveRequestFor: readArchiveRequestFor } = require('../scripts/read-archiveRequestFor');

readArchiveRequestFor('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
