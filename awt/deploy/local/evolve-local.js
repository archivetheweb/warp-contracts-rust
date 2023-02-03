const { evolve } = require('../scripts/evolve');

evolve('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
