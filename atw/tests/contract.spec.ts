import fs from 'fs';

import ArLocal from 'arlocal';
import Arweave from 'arweave';
import { JWKInterface } from 'arweave/node/lib/wallet';
import { LoggerFactory, Warp, SmartWeaveTags, WarpFactory, TagsParser } from 'warp-contracts';
import path from 'path';
import { AtwContract } from '../contract/definition/bindings/ts/AtwContract';
import { State } from '../contract/definition/bindings/ts/ContractState';
import { ArchiveSubmission } from '../contract/definition/bindings/ts/View';

jest.setTimeout(30000);

describe('Testing the ATW Contract', () => {
  let contractSrc: Buffer;

  let wallet: JWKInterface;
  let walletAddress: string;

  let initialState: State;

  let arweave: Arweave;
  let arlocal: ArLocal;
  let warp: Warp;
  let atwContract: AtwContract;
  let contractTxId: string;

  let tagsParser;

  let archiveSubmission: ArchiveSubmission = {
    options: {
      depth: 0,
      crawlType: 'domainOnly' // whether we want a domain only crawl
    },
    arweaveTx: '',
    fullUrl: 'https://example.com?hello=hi',
    size: 1,
    timestamp: 1,
    archiveRequestId: '',
    screenshotTx: '',
    title: '',
    uploaderAddress: ''
  };

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

      atwContract = new AtwContract(contractTxId, warp);

      // connecting wallet to the Atw contract
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

  it('should NOT allow a random uploader to submit', async () => {
    let ts = 1;
    let copiedArchiveSubmission = copyObject(archiveSubmission);
    copiedArchiveSubmission.timestamp = ts;
    copiedArchiveSubmission.archiveRequestId = '';
    await atwContract.submitArchive(copiedArchiveSubmission);

    const state = await atwContract.currentState();

    expect(Object.keys(state.archives).length).toEqual(0);
  });

  it('should register uploader', async () => {
    await atwContract.registerUploader({ friendlyName: 'bob' });
    const state = await atwContract.currentState();
    expect(Object.keys(state.uploaders).length).toEqual(1);
    expect(state.uploaders[walletAddress].friendlyName).toEqual('bob');
  });

  it('should add archiving request', async () => {
    await atwContract.requestArchiving({
      options: {
        urls: ['https://example.com'],
        depth: 0, // depth of the crawl
        crawlType: 'domainOnly' // whether we want a domain only crawl
      },
      uploaderAddress: walletAddress, // uploader for this pool
      startTimestamp: Math.floor(Date.now() / 1000), // start_timestamp of the period where we want to archive the website
      endTimestamp: Math.floor(Date.now() / 1000) + 60, // end_timestamp
      frequency: '0 * * * * * '
    });
    const state = await atwContract.currentState();

    expect(Object.keys(state.archiveRequests).length).toEqual(1);
  });

  it('should submit an archive', async () => {
    let beginState = await atwContract.currentState();
    let archiveRequestId = Object.keys(beginState.archiveRequests)[0];

    let beginCount = Object.keys(beginState.archives).length;
    let ts = 1;
    let copiedArchiveSubmission = copyObject(archiveSubmission);
    copiedArchiveSubmission.timestamp = ts;
    copiedArchiveSubmission.archiveRequestId = archiveRequestId;
    await atwContract.submitArchive(copiedArchiveSubmission);
    const state = await atwContract.currentState();

    expect(Object.keys(state.archives).length - beginCount).toEqual(1);

    expect(state.archives['example.com'][ts]).toBeDefined();
  });

  it('should submit a second archive', async () => {
    let beginState = await atwContract.currentState();
    let archiveRequestId = Object.keys(beginState.archiveRequests)[0];

    let beginCount = Object.keys(beginState.archives['example.com']).length;
    let ts = 2;
    let copiedArchiveSubmission = copyObject(archiveSubmission);
    copiedArchiveSubmission.timestamp = ts;
    copiedArchiveSubmission.archiveRequestId = archiveRequestId;
    await atwContract.submitArchive(copiedArchiveSubmission);
    const state = await atwContract.currentState();

    console.log(state);

    expect(Object.keys(state.archives['example.com']).length - beginCount).toEqual(1);

    expect(state.archiveRequests[archiveRequestId].latestArchivedTimestamp).toEqual(ts);

    expect(state.archives['example.com'][ts]).toBeDefined();
  });

  it('should delete an archive request', async () => {
    let beginState = await atwContract.currentState();
    let beginCount = Object.keys(beginState.archiveRequests).length;

    // we create an archive request
    let { originalTxId } = await atwContract.requestArchiving({
      options: {
        urls: ['https://example.com'],
        depth: 0, // depth of the crawl
        crawlType: 'domainOnly'
      },
      uploaderAddress: walletAddress, // uploader for this pool
      startTimestamp: Math.floor(Date.now() / 1000) - 2000, // start_timestamp of the period where we want to archive the website
      endTimestamp: Math.floor(Date.now() / 1000) - 1000, // end_timestamp
      frequency: '0 * * * * * '
    });

    await atwContract.deleteArchiveRequest({
      archiveId: originalTxId
    });
    const state = await atwContract.currentState();

    expect(beginCount - Object.keys(state.archiveRequests).length).toEqual(0);
  });

  it('should deregister an uploader', async () => {
    let beginState = await atwContract.currentState();
    let beginCount = Object.keys(beginState.uploaders).length;

    // we create an archive request
    let { originalTxId } = await atwContract.deRegisterUploader({});

    const state = await atwContract.currentState();

    expect(beginCount - Object.keys(state.uploaders).length).toEqual(1);
    expect(state.uploaders[walletAddress]).toBeUndefined();
  });
});

function copyObject(object: any): any {
  return JSON.parse(JSON.stringify(object));
}
