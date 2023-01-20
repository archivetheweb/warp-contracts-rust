import fs from 'fs';

import ArLocal from 'arlocal';
import Arweave from 'arweave';
import { JWKInterface } from 'arweave/node/lib/wallet';
import { InteractionResult, LoggerFactory, Warp, SmartWeaveTags, WarpFactory, TagsParser } from 'warp-contracts';
import path from 'path';
import { AwtContract } from '../contract/definition/bindings/ts/AwtContract';
import { State } from '../contract/definition/bindings/ts/ContractState';

jest.setTimeout(30000);

describe('Testing the ATW Contract', () => {
  let contractSrc: Buffer;

  let wallet: JWKInterface;
  let walletAddress: string;

  let initialState: State;

  let arweave: Arweave;
  let arlocal: ArLocal;
  let warp: Warp;
  let atwContract: AwtContract;
  let contractTxId: string;

  let tagsParser;

  beforeAll(async () => {
    // note: each tests suit (i.e. file with tests that Jest is running concurrently
    // with another files has to have ArLocal set to a different port!)
    arlocal = new ArLocal(1821, false);
    await arlocal.start();

    tagsParser = new TagsParser();

    LoggerFactory.INST.logLevel('error');
    LoggerFactory.INST.logLevel('debug', 'WASM:Rust');
    //LoggerFactory.INST.logLevel('debug', 'WasmContractHandlerApi');

    warp = WarpFactory.forLocal(1821);
    arweave = warp.arweave;

    ({ jwk: wallet, address: walletAddress } = await warp.generateWallet());

    contractSrc = fs.readFileSync(path.join(__dirname, '../contract/implementation/pkg/rust-contract_bg.wasm'));
    const stateFromFile: State = JSON.parse(fs.readFileSync(path.join(__dirname, './data/test_state.json'), 'utf8'));

    initialState = {
      ...stateFromFile,
      ...{
        owner: walletAddress
      }
    };

    try {
      // deploying contract using the new SDK.
      let { contractTxId: ctxID } = await warp.createContract.deploy({
        wallet,
        initState: JSON.stringify(initialState),
        src: contractSrc,
        wasmSrcCodeDir: path.join(__dirname, '../contract/implementation/src'),
        wasmGlueCode: path.join(__dirname, '../contract/implementation/pkg/rust-contract.js')
      });

      contractTxId = ctxID;

      atwContract = new AwtContract(contractTxId, warp);

      // connecting wallet to the Awt contract
      atwContract.connect(wallet).setEvaluationOptions({ internalWrites: true });
    } catch (e) {
      console.error(e);
    }
  });

  afterAll(async () => {
    await arlocal.stop();
  });

  it('should properly deploy contract', async () => {
    const contractTx = await arweave.transactions.get(contractTxId);

    expect(contractTx).not.toBeNull();

    const contractSrcTx = await arweave.transactions.get(
      tagsParser.getTag(contractTx, SmartWeaveTags.CONTRACT_SRC_TX_ID)
    );
    expect(tagsParser.getTag(contractSrcTx, SmartWeaveTags.CONTENT_TYPE)).toEqual('application/wasm');
    expect(tagsParser.getTag(contractSrcTx, SmartWeaveTags.WASM_LANG)).toEqual('rust');
  });

  it('should register uploader', async () => {
    await atwContract.registerUploader({ friendlyName: 'bob' });
    const state = await atwContract.currentState();
    expect(Object.keys(state.uploaders).length).toEqual(1);
    expect(state.uploaders[walletAddress].friendlyName).toEqual('bob');
  });

  it('should add archiving request', async () => {
    await atwContract.requestArchiving({
      crawlOptions: {
        urls: ['https://example.com'],
        depth: 0, // depth of the crawl
        domainOnly: false // whether we want a domain only crawl
      },
      uploaderAddress: walletAddress, // uploader for this pool
      startTimestamp: Math.floor(Date.now() / 1000), // start_timestamp of the period where we want to archive the website
      endTimestamp: Math.floor(Date.now() / 1000), // end_timestamp
      frequency: 'cron:0 * * * * * '
    });
    const state = await atwContract.currentState();

    expect(Object.keys(state.archivingRequests).length).toEqual(1);
  });
});
