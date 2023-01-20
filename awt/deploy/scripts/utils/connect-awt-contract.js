const { getWarpInstance } = require('../deploy');

module.exports.connectAWTContract = async function (arweave, wallet, contractTxId, target) {
  const { AwtContract } = require('../../../contract/definition/bindings/ts/AwtContract');
  const warp = getWarpInstance(arweave.api.config.port, target);
  let awtContract = new AwtContract(contractTxId, warp).connect(wallet);
  awtContract.setEvaluationOptions({ internalWrites: true });
  return awtContract;
};
