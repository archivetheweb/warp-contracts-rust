const fs = require('fs');
const path = require('path');
const { mineBlock } = require('./utils/mine-block');
const { loadWallet, walletAddress } = require('./utils/load-wallet');
const { connectArweave } = require('./utils/connect-arweave');
const { contractTxId } = require('./utils/contract-tx-id');
const { connectATWContract } = require('./utils/connect-atw-contract');
const { getWarpInstance } = require('./deploy');

module.exports.evolve = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const warp = getWarpInstance(port, target);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const contractSrc = fs.readFileSync(path.join(__dirname, '../../contract/implementation/pkg/rust-contract_bg.wasm'));
  const txId = contractTxId(target);
  const atw = await connectATWContract(arweave, wallet, txId, target);

  const srcTx = await warp.createSourceTx(
    {
      src: contractSrc,
      wasmSrcCodeDir: path.join(__dirname, '../../src'),
      wasmGlueCode: path.join(__dirname, '../../contract/implementation/pkg/rust-contract.js')
    },
    wallet
  );
  const newContractTxId = await warp.saveSourceTx(srcTx);

  await atw.evolve({ value: newContractTxId });

  if (target == 'testnet' || target == 'local') {
    await mineBlock(arweave);
  }

  let afterState = await atw.currentState();
  console.log('new evolve:', afterState.evolve);

  if (target == 'testnet') {
    console.log(`Check contract at https://sonar.warp.cc/#/app/contract/${contractTxId}?network=testnet`);
  } else {
    console.log('Contract tx id', contractTxId);
  }
};
