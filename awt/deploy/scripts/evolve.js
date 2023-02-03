const fs = require('fs');
const path = require('path');
const { mineBlock } = require('./utils/mine-block');
const { loadWallet, walletAddress } = require('./utils/load-wallet');
const { connectArweave } = require('./utils/connect-arweave');
const { contractTxId } = require('./utils/contract-tx-id');
const { connectAWTContract } = require('./utils/connect-awt-contract');
const { getWarpInstance } = require('./deploy');

module.exports.evolve = async function (host, port, protocol, target, walletJwk) {
  const arweave = connectArweave(host, port, protocol);
  const warp = getWarpInstance(port, target);
  const wallet = await loadWallet(arweave, walletJwk, target, true);
  const contractSrc = fs.readFileSync(path.join(__dirname, '../../contract/implementation/pkg/rust-contract_bg.wasm'));
  const txId = contractTxId(target);
  const awt = await connectAWTContract(arweave, wallet, txId, target);

  let state = await awt.currentState();
  console.log('current evolve:', state.evolve);
  const srcTx = await warp.createSourceTx(
    {
      src: contractSrc,
      wasmSrcCodeDir: path.join(__dirname, '../../src'),
      wasmGlueCode: path.join(__dirname, '../../contract/implementation/pkg/rust-contract.js')
    },
    wallet
  );
  const newContractTxId = await warp.saveSourceTx(srcTx, true);

  await awt.evolve({ value: newContractTxId });

  await fs.writeFileSync(path.join(__dirname, `../${target}/contract-tx-id.txt`), newContractTxId);

  if (target == 'testnet' || target == 'local') {
    await mineBlock(arweave);
  }

  let afterState = await awt.currentState();
  console.log('new evolve:', afterState.evolve);

  if (target == 'testnet') {
    console.log(`Check contract at https://sonar.warp.cc/#/app/contract/${contractTxId}?network=testnet`);
  } else {
    console.log('Contract tx id', contractTxId);
  }
};
