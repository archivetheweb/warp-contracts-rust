const { readContractState } = require('../scripts/read');

readContractState('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
