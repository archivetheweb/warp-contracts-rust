const { requestArchiving: requestArchiving } = require('../scripts/interact');

requestArchiving('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
