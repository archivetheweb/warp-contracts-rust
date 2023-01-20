const { readArchiveRequestFor: readArchiveRequestFor } = require('../scripts/read');

readArchiveRequestFor('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
