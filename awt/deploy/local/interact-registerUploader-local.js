const { registerUploader: registerUploader } = require('../scripts/interact-registerUploader');

registerUploader('localhost', 1984, 'http', 'local', 'deploy/local/wallet_local.json').finally();
