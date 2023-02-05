const { loadWallet } = require('./utils/load-wallet');
const { connectArweave } = require('./utils/connect-arweave');
const { connectContract } = require('./utils/connect-contract');
const { contractTxId } = require(`./utils/contract-tx-id`);
const { connectAWTContract } = require('./utils/connect-awt-contract');

module.exports.readContractState = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);

  const txId = contractTxId(target);
  const contract = await connectContract(arweave, wallet, txId, target);
  const { cachedValue } = await contract.readState();

  console.log('Current state:', cachedValue.state);
  console.log('Contract tx id', txId);
};

module.exports.readArchiveRequestFor = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);

  const walletAddress = await arweave.wallets.jwkToAddress(wallet);
  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);
  const archiveRequestsFor = await awt.archiveRequestsFor({ address: walletAddress });

  console.log(archiveRequestsFor);
};

module.exports.archivesByURL = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);

  const walletAddress = await arweave.wallets.jwkToAddress(wallet);

  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);
  const archivesByURL = await awt.archivesByURL({ url: 'https://example.com', count: 10 });

  console.log(archivesByURL);
};
