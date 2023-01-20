const { loadWallet, walletAddress } = require('./utils/load-wallet');
const { connectArweave } = require('./utils/connect-arweave');
const { connectAWTContract } = require('./utils/connect-awt-contract');
const { contractTxId } = require('./utils/contract-tx-id');
const { mineBlock } = require('./utils/mine-block');

module.exports.registerUploader = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);

  const { originalTxId } = await awt.registerUploader({ friendlyName: 'bob' });

  await mineBlock(arweave);
  const state = await awt.currentState();

  console.log('Updated state:', state);
  console.log('Contract tx id', txId);

  if (target == 'testnet') {
    console.log(
      `Check registerUploader interaction at https://sonar.warp.cc/#/app/interaction/${originalTxId}?network=testnet`
    );
  } else {
    console.log('Transfer tx id', originalTxId);
  }
};

module.exports.requestArchiving = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const walletAddr = await walletAddress(arweave, wallet);
  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);

  const { originalTxId } = await awt.requestArchiving({
    crawlOptions: { depth: 0, urls: ['https://example.com'], domainOnly: false },
    endTimestamp: 2,
    frequency: 'freq',
    startTimestamp: 1,
    uploaderAddress: walletAddr
  });

  await mineBlock(arweave);
  const state = await awt.currentState();

  console.log('Updated state:', state);
  console.log('Contract tx id', txId);

  if (target == 'testnet') {
    console.log(
      `Check requestArchiving interaction at https://sonar.warp.cc/#/app/interaction/${originalTxId}?network=testnet`
    );
  } else {
    console.log('Transfer tx id', originalTxId);
  }
};

module.exports.submitArchive = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const walletAddr = await walletAddress(arweave, wallet);
  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);

  let beginState = await awt.currentState();
  let key = Object.keys(beginState.archivingRequests)[0];

  const { originalTxId } = await awt.submitArchive({
    archiveRequestId: key,
    arweaveTx: 'string',
    fullUrl: 'https://example.com',
    info: { depth: 0, urls: ['https://example.com'], domainOnly: false },
    size: 1,
    timestamp: 1
  });

  await mineBlock(arweave);
  const state = await awt.currentState();

  console.log('Updated state:', state);
  console.log('Contract tx id', txId);

  if (target == 'testnet') {
    console.log(
      `Check requestArchiving interaction at https://sonar.warp.cc/#/app/interaction/${originalTxId}?network=testnet`
    );
  } else {
    console.log('Transfer tx id', originalTxId);
  }
};
