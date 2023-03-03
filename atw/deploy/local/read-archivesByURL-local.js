const { archivesByURL: archivesByURL } = require('../scripts/read');

archivesByURL('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
