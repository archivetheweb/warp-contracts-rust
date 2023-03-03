const { registerUploader: registerUploader } = require('../scripts/interact');

registerUploader('arweave.net', 443, 'https', 'mainnet', 'deploy/mainnet/wallet_mainnet.json').finally();
