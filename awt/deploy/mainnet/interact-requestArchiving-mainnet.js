const { requestArchiving: requestArchiving } = require('../scripts/interact');

requestArchiving('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
