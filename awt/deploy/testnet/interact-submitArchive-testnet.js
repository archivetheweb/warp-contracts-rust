const { submitAr, submitArchivechive: submitArchive } = require('../scripts/interact');

submitArchive('sonar.warp.cc', 443, 'https', 'testnet', 'deploy/testnet/wallet_testnet.json').finally();
