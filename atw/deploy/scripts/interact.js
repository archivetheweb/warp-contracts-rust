const { loadWallet, walletAddress } = require('./utils/load-wallet');
const { connectArweave } = require('./utils/connect-arweave');
const { connectATWContract } = require('./utils/connect-atw-contract');
const { contractTxId } = require('./utils/contract-tx-id');
const { mineBlock } = require('./utils/mine-block');

module.exports.registerUploader = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const txId = contractTxId(target);
  const atw = await connectATWContract(arweave, wallet, txId, target);

  const { originalTxId } = await atw.registerUploader({ friendlyName: 'bob' });

  await mineBlock(arweave);
  const state = await atw.currentState();

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
  const atw = await connectATWContract(arweave, wallet, txId, target);

  const { originalTxId } = await atw.requestArchiving({
    options: { depth: 0, urls: ['https://example.com'], domainOnly: false },
    // for about a day
    endTimestamp: Math.floor(Date.now() / 1000) + 100000,
    // every 10 minutes
    frequency: '0 0/10 * * * *',
    startTimestamp: Math.floor(Date.now() / 1000),
    uploaderAddress: walletAddr
  });

  await mineBlock(arweave);
  const state = await atw.currentState();

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
  const atw = await connectATWContract(arweave, wallet, txId, target);

  let beginState = await atw.currentState();
  let key = Object.keys(beginState.archiveRequests)[0];

  const { originalTxId } = await atw.submitArchive({
    archiveRequestId: key,
    arweaveTx: 'string',
    fullUrl: 'https://example.com',
    options: { depth: 0, urls: ['https://example.com'], domainOnly: false },
    size: 1,
    timestamp: 1
  });

  await mineBlock(arweave);
  const state = await atw.currentState();

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
