const { requestArchiving: requestArchiving } = require('../scripts/interact');

requestArchiving('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
