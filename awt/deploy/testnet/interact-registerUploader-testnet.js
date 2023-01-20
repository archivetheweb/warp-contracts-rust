const { registerUploader: registerUploader } = require('../scripts/interact');

registerUploader('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
