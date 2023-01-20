const { registerUploader: registerUploader } = require('../scripts/interact');

registerUploader('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
