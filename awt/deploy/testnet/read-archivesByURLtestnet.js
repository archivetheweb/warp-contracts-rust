const { archivesByURL: archivesByURL } = require('../scripts/read');

archivesByURL('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
