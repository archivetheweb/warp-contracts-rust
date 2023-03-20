const { upgradeState: upgradeState } = require('../scripts/interact');

upgradeState('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
