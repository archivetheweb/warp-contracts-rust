const { readArchiveRequestFor: readArchiveRequestFor } = require('../scripts/read');

readArchiveRequestFor('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
