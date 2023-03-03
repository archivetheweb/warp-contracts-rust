const { getWarpInstance } = require('../deploy');

module.exports.connectAtwContract = async function (arweave, wallet, contractTxId, target) {
  const { AtwContract } = require('../../../contract/definition/bindings/ts/AtwContract');
  const warp = getWarpInstance(arweave.api.config.port, target);
  let atwContract = new AtwContract(contractTxId, warp).connect(wallet);
  atwContract.setEvaluationOptions({ internalWrites: true });
  return atwContract;
};
