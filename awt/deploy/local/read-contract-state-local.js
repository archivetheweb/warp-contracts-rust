const { readContractState } = require('../scripts/read');

readContractState('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
